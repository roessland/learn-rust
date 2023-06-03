package person_test

import (
	"fmt"
	"my-module/person"
	"testing"
)

func TestPerson(t *testing.T) {
	p, err := person.New("John", "Doe")
	fmt.Println(p, err)
}
