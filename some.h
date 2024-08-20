#pragma once

struct Platform;

typedef struct {
	/* Platform; something that comes from Rust application level
	*/
	struct Platform *platform_p;
	int count;
} Context;

void tunnel(Context *p);
