#include <libpmemobj/base.h>
#include <libpmemobj/tx.h>
#include <stdbool.h>
#include <stdio.h>

POBJ_LAYOUT_BEGIN(mylayout);
POBJ_LAYOUT_ROOT(mylayout, struct root);
POBJ_LAYOUT_TOID(mylayout, struct node);
POBJ_LAYOUT_TOID(mylayout, struct foo);
POBJ_LAYOUT_END(mylayout);

struct root
{
	TOID(struct node) node;
};

struct node
{
	TOID(struct node) next;
	TOID(struct foo) foo;
};

//The name of layout and the number of declared types can be retrieved using the following code:

const char *layout_name = POBJ_LAYOUT_NAME(mylayout);
int num_of_types = POBJ_LAYOUT_TYPES_NUM(mylayout);

// https://github.com/pmem/nvml/blob/master/doc/libpmemobj.3.md#caveats
bool transaction()
{
	bool volatile result;

	TX_BEGIN(pop) {

		// This block is longjmp-safe

		fputs("Transaction Begin - do work here", stderr);

	} TX_ONCOMMIT {

		// This block is longjmp-safe

		fputs("Transaction Commit", stderr);
		result = true;

	} TX_ONABORT {

		// This block IS NOT longjmp-safe

		fputs("Transaction Abort", stderr);
		result = false;

	} TX_FINALLY {

		// This block IS NOT longjmp-safe if transaction aborted (in TX_BEGIN)
		// This block is longjmp-safe if transaction committed

		fputs("Transaction Finally", stderr);

	} TX_END

	// This block IS NOT longjmp-safe

	return result;
}

bool transaction2()
{
	int * volatile good_example = (int *)0xBAADF00D;

	bool volatile result;

	TX_BEGIN(pop) {

		// This block is longjmp-safe

		fputs("Transaction Begin - do work here", stderr);

		good_example = malloc(sizeof(int));

	} TX_ONCOMMIT {

		// This block is longjmp-safe

		fputs("Transaction Commit", stderr);
		result = true;

		free(good_example);

	} TX_ONABORT {

		// This block IS NOT longjmp-safe

		fputs("Transaction Abort", stderr);
		result = false;

	} TX_FINALLY {

		// This block IS NOT longjmp-safe if transaction aborted (in TX_BEGIN)
		// This block is longjmp-safe if transaction committed

		fputs("Transaction Finally", stderr);

	} TX_END

	// This block IS NOT longjmp-safe
	return result;
}
