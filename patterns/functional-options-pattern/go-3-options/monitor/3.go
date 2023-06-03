package monitor

import (
	"fmt"
)

type Resolution struct {
	Width  int
	Weight int
}

type Monitor struct {
	resolution Resolution
	hasStand   bool
	tech       string
}

func validate(m *Monitor) error {
	if m.tech == "" {
		return fmt.Errorf("tech is required")
	}
	if m.resolution.Width <= 0 || m.resolution.Weight <= 0 {
		return fmt.Errorf("resolution %v is invalid", m.resolution)
	}
	return nil
}

func New(options ...func(*Monitor)) (*Monitor, error) {
	m := Monitor{}
	for _, option := range options {
		option(&m)
	}
	if err := validate(&m); err != nil {
		return nil, err
	}
	return &m, nil
}

func NewSimple(resolution Resolution, hasStand bool, tech string) (*Monitor, error) {
	m := Monitor{
		resolution: resolution,
		hasStand:   false,
		tech:       "",
	}
	if err := validate(&m); err != nil {
		return nil, err
	}
	return &m, nil
}

func WithStand() func(*Monitor) {
	return func(monitor *Monitor) {
		monitor.hasStand = true
	}
}

var WithoutStand = func(monitor *Monitor) {
	monitor.hasStand = false
}

func WithTech(tech string) func(*Monitor) {
	return func(monitor *Monitor) {
		monitor.tech = tech
	}
}

func WithResolution(width, height int) func(*Monitor) {
	return func(monitor *Monitor) {
		monitor.resolution = Resolution{width, height}
	}
}
