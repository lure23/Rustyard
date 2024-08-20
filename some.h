#pragma once

typedef struct {
	/* No use acting like we know which (Rust) type is behind that pointer. We don't.
	*/
	void *vp;
	int count;
} Context;

void tunnel(Context *p);
