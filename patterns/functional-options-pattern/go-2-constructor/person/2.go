package person

import "fmt"

type Person struct {
	firstName string
	lastName  string
}

func New(firstName, lastName string) (*Person, error) {
	person := &Person{firstName, lastName}
	if err := validatePerson(person); err != nil {
		return nil, err
	}
	return person, nil
}

func validatePerson(person *Person) error {
	if person.firstName == "" {
		return fmt.Errorf("firstName is required")
	}
	if person.lastName == "" {
		return fmt.Errorf("lastName is required")
	}
	return nil
}
