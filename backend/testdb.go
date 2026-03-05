//go:build ignore

package main

import (
	"fmt"
	"log"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Service struct {
	ID   int
	Name string
	Type string
}

type MediaType struct {
	ID   int
	Name string
}

type Classification struct {
	ID   int
	Name string
}

type Media struct {
	ID               int
	Title            string
	MediaTypeID      int
	Description      string
	DurationSeconds  int
	Genre            string
	ClassificationID *int
}

type MediaService struct {
	MediaID   int `gorm:"primaryKey"`
	ServiceID int `gorm:"primaryKey"`
}

type Customer struct {
	ID       int
	Username string
}

type Subscription struct {
	ID            int
	CustomerID    int
	ServiceID     int
	StartedAt     string
	ExpiresAt     *string
	BillingPeriod int
}

func main() {
	dsn := "host=localhost user=postgres password=mypassword dbname=testflix port=5432 sslmode=disable"
	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Connected to testflix!")

	// Services
	var services []Service
	db.Find(&services)
	fmt.Println("\n--- services ---")
	for _, s := range services {
		fmt.Printf("  id=%d  name=%q  type=%s\n", s.ID, s.Name, s.Type)
	}

	// Media types
	var mediaTypes []MediaType
	db.Find(&mediaTypes)
	fmt.Println("\n--- media_types ---")
	for _, mt := range mediaTypes {
		fmt.Printf("  id=%d  name=%s\n", mt.ID, mt.Name)
	}

	// Classifications
	var classifications []Classification
	db.Find(&classifications)
	fmt.Println("\n--- classifications ---")
	for _, c := range classifications {
		fmt.Printf("  id=%d  name=%s\n", c.ID, c.Name)
	}

	// Media
	var media []Media
	db.Find(&media)
	fmt.Println("\n--- media ---")
	for _, m := range media {
		classID := 0
		if m.ClassificationID != nil {
			classID = *m.ClassificationID
		}
		fmt.Printf("  id=%d  title=%q  media_type_id=%d  genre=%s  classification_id=%d  duration=%ds\n",
			m.ID, m.Title, m.MediaTypeID, m.Genre, classID, m.DurationSeconds)
	}

	// Media-service mapping
	var mediaServices []MediaService
	db.Find(&mediaServices)
	fmt.Println("\n--- media_services ---")
	for _, ms := range mediaServices {
		fmt.Printf("  media_id=%d  service_id=%d\n", ms.MediaID, ms.ServiceID)
	}

	// Customers
	var customers []Customer
	db.Find(&customers)
	fmt.Println("\n--- customers ---")
	for _, c := range customers {
		fmt.Printf("  id=%d  username=%s\n", c.ID, c.Username)
	}

	// Subscriptions
	var subscriptions []Subscription
	db.Find(&subscriptions)
	fmt.Println("\n--- subscriptions ---")
	for _, s := range subscriptions {
		expires := "NULL"
		if s.ExpiresAt != nil {
			expires = *s.ExpiresAt
		}
		fmt.Printf("  id=%d  customer_id=%d  service_id=%d  billing_period=%d  started_at=%s  expires_at=%s\n",
			s.ID, s.CustomerID, s.ServiceID, s.BillingPeriod, s.StartedAt, expires)
	}
}
