package allergies

func Allergies(score int) []string {
	allergies := []string{}

	if (score & 1) == 1 {
		allergies = append(allergies, "eggs")
	}
	if (score & 2) == 2 {
		allergies = append(allergies, "peanuts")
	}
	if (score & 4) == 4 {
		allergies = append(allergies, "shellfish")
	}

	if (score & 8) == 8 {
		allergies = append(allergies, "strawberries")
	}

	if (score & 16) == 16 {
		allergies = append(allergies, "tomatoes")
	}

	if (score & 32) == 32 {
		allergies = append(allergies, "chocolate")
	}

	if (score & 64) == 64 {
		allergies = append(allergies, "pollen")
	}

	if (score & 128) == 128 {
		allergies = append(allergies, "cats")
	}

	return allergies
}

func AllergicTo(score int, allergen string) bool {
	allergies := Allergies(score)

	for _, possible_allergen := range allergies {
		if allergen == possible_allergen {
			return true
		}
	}
	return false
}
