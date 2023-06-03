package person_test

import (
	"fmt"
	"my-module/person"
	"testing"
)

func TestPerson(t *testing.T) {
	p := &person.Person{FirstName: "John", LastName: "Doe"}
	fmt.Println(p)
}
