package monitor_test

import (
	"fmt"
	"my-module/monitor"
	"testing"
)

func TestMonitor(t *testing.T) {
	mon, err := monitor.New(
		monitor.WithStand(),
		monitor.WithTech("OLED"),
		monitor.WithResolution(1920, 1080),
		monitor.WithoutStand,
	)
	fmt.Printf("%v - %v\n", mon, err)
}

func TestMonitor_Simple(t *testing.T) {
	mon, err := monitor.NewSimple(
		monitor.Resolution{Width: 1920, Weight: 1080},
		true,
		"OLED",
	)
	fmt.Printf("%v - %v\n", mon, err)
}
