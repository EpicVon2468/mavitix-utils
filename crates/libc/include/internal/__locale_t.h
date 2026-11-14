#pragma once
#include <stdint.h>

typedef enum __language_t {
	English,
	Japanese,
} __language_t;

typedef enum __territory_t {
	None,
	AU,
	GB,
	JP,
	NZ,
} __territory_t;

typedef struct locale_t {
	__language_t language;
	__territory_t territory;
} *locale_t;
