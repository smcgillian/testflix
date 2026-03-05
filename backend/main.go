package main

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strconv"

	_ "github.com/jackc/pgx/v5/stdlib"
)

var db *sql.DB

func main() {
	var err error
	dsn := "host=localhost user=postgres password=mypassword dbname=testflix port=5432 sslmode=disable"
	db, err = sql.Open("pgx", dsn)
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		log.Fatal(err)
	}
	log.Println("Connected to testflix database")

	mux := http.NewServeMux()

	// API routes
	mux.HandleFunc("GET /api/media", handleListMedia)
	mux.HandleFunc("GET /api/media/{id}", handleGetMedia)
	mux.HandleFunc("GET /api/services", handleListServices)
	mux.HandleFunc("GET /api/customers/{email}", handleGetCustomer)
	mux.HandleFunc("GET /api/customers/{email}/subscriptions", handleListSubscriptions)
	mux.HandleFunc("PUT /api/subscriptions/{id}", handleUpdateSubscription)

	// Static frontend files
	mux.Handle("/", http.FileServer(http.Dir("../frontend")))

	fmt.Println("Server listening on :8080")
	log.Fatal(http.ListenAndServe(":8080", mux))
}

// --- helpers ---

func jsonError(w http.ResponseWriter, msg string, code int) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)
	json.NewEncoder(w).Encode(map[string]string{"error": msg})
}

func jsonOK(w http.ResponseWriter, data any) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]any{"data": data})
}

func pathID(r *http.Request, name string) (int, error) {
	return strconv.Atoi(r.PathValue(name))
}

// --- GET /media?email={email} ---

func handleListMedia(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if email == "" {
		jsonError(w, "email query parameter is required", http.StatusBadRequest)
		return
	}

	var customerID int
	err := db.QueryRow("SELECT id FROM customers WHERE email = $1", email).Scan(&customerID)
	if err == sql.ErrNoRows {
		jsonError(w, "customer not found", http.StatusNotFound)
		return
	} else if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	rows, err := db.Query(`
		SELECT m.id, m.title, mt.name, m.description, m.duration_seconds, m.genre, COALESCE(c.name, '')
		FROM media m
		JOIN media_services ms ON ms.media_id = m.id
		JOIN subscriptions s ON s.service_id = ms.service_id
		JOIN media_types mt ON mt.id = m.media_type_id
		LEFT JOIN classifications c ON c.id = m.classification_id
		WHERE s.customer_id = $1
		ORDER BY m.id
	`, customerID)
	if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	type mediaItem struct {
		ID              int    `json:"id"`
		Title           string `json:"title"`
		MediaType       string `json:"media_type"`
		Description     string `json:"description"`
		DurationSeconds int    `json:"duration_seconds"`
		Genre           string `json:"genre"`
		Classification  string `json:"classification"`
	}

	items := []mediaItem{}
	for rows.Next() {
		var m mediaItem
		if err := rows.Scan(&m.ID, &m.Title, &m.MediaType, &m.Description, &m.DurationSeconds, &m.Genre, &m.Classification); err != nil {
			jsonError(w, "internal server error", http.StatusInternalServerError)
			return
		}
		items = append(items, m)
	}

	jsonOK(w, items)
}

// --- GET /media/{id} ---

func handleGetMedia(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r, "id")
	if err != nil {
		jsonError(w, "invalid media id", http.StatusBadRequest)
		return
	}

	type mediaDetail struct {
		ID              int      `json:"id"`
		Title           string   `json:"title"`
		MediaType       string   `json:"media_type"`
		Description     string   `json:"description"`
		DurationSeconds int      `json:"duration_seconds"`
		Genre           string   `json:"genre"`
		Classification  string   `json:"classification"`
		Services        []string `json:"services"`
	}

	var m mediaDetail
	err = db.QueryRow(`
		SELECT m.id, m.title, mt.name, m.description, m.duration_seconds, m.genre, COALESCE(c.name, '')
		FROM media m
		JOIN media_types mt ON mt.id = m.media_type_id
		LEFT JOIN classifications c ON c.id = m.classification_id
		WHERE m.id = $1
	`, id).Scan(&m.ID, &m.Title, &m.MediaType, &m.Description, &m.DurationSeconds, &m.Genre, &m.Classification)
	if err == sql.ErrNoRows {
		jsonError(w, "media not found", http.StatusNotFound)
		return
	} else if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	rows, err := db.Query(`
		SELECT s.name FROM services s
		JOIN media_services ms ON ms.service_id = s.id
		WHERE ms.media_id = $1
	`, id)
	if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	m.Services = []string{}
	for rows.Next() {
		var name string
		if err := rows.Scan(&name); err != nil {
			jsonError(w, "internal server error", http.StatusInternalServerError)
			return
		}
		m.Services = append(m.Services, name)
	}

	jsonOK(w, m)
}

// --- GET /services ---

func handleListServices(w http.ResponseWriter, r *http.Request) {
	rows, err := db.Query("SELECT id, name, type FROM services ORDER BY id")
	if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	type service struct {
		ID   int    `json:"id"`
		Name string `json:"name"`
		Type string `json:"type"`
	}

	services := []service{}
	for rows.Next() {
		var s service
		if err := rows.Scan(&s.ID, &s.Name, &s.Type); err != nil {
			jsonError(w, "internal server error", http.StatusInternalServerError)
			return
		}
		services = append(services, s)
	}

	jsonOK(w, services)
}

// --- GET /customers/{email} ---

func handleGetCustomer(w http.ResponseWriter, r *http.Request) {
	email := r.PathValue("email")

	type customer struct {
		ID        int    `json:"id"`
		FirstName string `json:"first_name"`
		LastName  string `json:"last_name"`
		Email     string `json:"email"`
	}

	var c customer
	err := db.QueryRow("SELECT id, first_name, last_name, email FROM customers WHERE email = $1", email).Scan(&c.ID, &c.FirstName, &c.LastName, &c.Email)
	if err == sql.ErrNoRows {
		jsonError(w, "customer not found", http.StatusNotFound)
		return
	} else if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	jsonOK(w, c)
}

// --- GET /customers/{email}/subscriptions ---

func handleListSubscriptions(w http.ResponseWriter, r *http.Request) {
	email := r.PathValue("email")

	var customerID int
	err := db.QueryRow("SELECT id FROM customers WHERE email = $1", email).Scan(&customerID)
	if err == sql.ErrNoRows {
		jsonError(w, "customer not found", http.StatusNotFound)
		return
	} else if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	rows, err := db.Query(`
		SELECT sub.id, sub.customer_id, sub.service_id, s.name, s.type,
		       sub.billing_period, sub.started_at, sub.expires_at
		FROM subscriptions sub
		JOIN services s ON s.id = sub.service_id
		WHERE sub.customer_id = $1
		ORDER BY sub.id
	`, customerID)
	if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	type subscription struct {
		ID            int     `json:"id"`
		CustomerID    int     `json:"customer_id"`
		ServiceID     int     `json:"service_id"`
		ServiceName   string  `json:"service_name"`
		ServiceType   string  `json:"service_type"`
		BillingPeriod int     `json:"billing_period"`
		StartedAt     string  `json:"started_at"`
		ExpiresAt     *string `json:"expires_at"`
	}

	subs := []subscription{}
	for rows.Next() {
		var s subscription
		if err := rows.Scan(&s.ID, &s.CustomerID, &s.ServiceID, &s.ServiceName, &s.ServiceType,
			&s.BillingPeriod, &s.StartedAt, &s.ExpiresAt); err != nil {
			jsonError(w, "internal server error", http.StatusInternalServerError)
			return
		}
		subs = append(subs, s)
	}

	jsonOK(w, subs)
}

// --- PUT /subscriptions/{id} ---

func handleUpdateSubscription(w http.ResponseWriter, r *http.Request) {
	id, err := pathID(r, "id")
	if err != nil {
		jsonError(w, "invalid subscription id", http.StatusBadRequest)
		return
	}

	type req struct {
		ServiceID     int `json:"service_id"`
		BillingPeriod int `json:"billing_period"`
	}

	var body req
	if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
		jsonError(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if body.BillingPeriod != 1 && body.BillingPeriod != 2 {
		jsonError(w, "billing_period must be 1 or 2", http.StatusBadRequest)
		return
	}

	// validate service exists
	var exists bool
	if err := db.QueryRow("SELECT EXISTS(SELECT 1 FROM services WHERE id = $1)", body.ServiceID).Scan(&exists); err != nil || !exists {
		jsonError(w, "service not found", http.StatusNotFound)
		return
	}

	type subscription struct {
		ID            int     `json:"id"`
		CustomerID    int     `json:"customer_id"`
		ServiceID     int     `json:"service_id"`
		BillingPeriod int     `json:"billing_period"`
		StartedAt     string  `json:"started_at"`
		ExpiresAt     *string `json:"expires_at"`
	}

	var sub subscription
	err = db.QueryRow(`
		UPDATE subscriptions
		SET service_id = $1, billing_period = $2, started_at = now()
		WHERE id = $3
		RETURNING id, customer_id, service_id, billing_period, started_at, expires_at
	`, body.ServiceID, body.BillingPeriod, id).Scan(
		&sub.ID, &sub.CustomerID, &sub.ServiceID, &sub.BillingPeriod, &sub.StartedAt, &sub.ExpiresAt)
	if err == sql.ErrNoRows {
		jsonError(w, "subscription not found", http.StatusNotFound)
		return
	} else if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	jsonOK(w, sub)
}
