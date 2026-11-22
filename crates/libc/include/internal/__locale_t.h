#pragma once

typedef enum __language_t {
	English,  /* English */
	German,	  /* Deutsch */
	Japanese, /* 日本語 */
} __language_t;

typedef enum __territory_t {
	None, /* Unspecified */
	AQ,	  /* Antarctica */
	AU,	  /* Australia */
	DE,	  /* Germany; Deutschland */
	GB,	  /* [The] United Kingdom [of Great Britain and Northern Ireland] */
	JP,	  /* Japan; 日本 */
	NZ,	  /* New Zealand */
} __territory_t;

typedef struct locale_t {
	__language_t language;
	__territory_t territory;
} *locale_t;
