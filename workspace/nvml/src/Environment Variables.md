# Environment Variables

## License

This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at <https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT>. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at <https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT>.


## Environment variables

### For Libraries

```
Library			#define							Name					X	Notes
libpmem											PMEM_IS_PMEM_FORCE		?	Used to make normal memory appear to be persistent memory
libpmem											PMEM_NO_CLFLUSHOPT		?	No current production use cases
libpmem											PMEM_NO_CLWB			?	No current production use cases
libpmem											PMEM_NO_FLUSH			?	No current production use cases
libpmem											PMEM_MOVNT_THRESHOLD	?	Tweak
libpmem											PMEM_NO_MOVNT			?	No current production use cases
libpmem			PMEM_LOG_LEVEL_VAR				PMEM_LOG_LEVEL			N	Common initialisation of logging\*
libpmem			PMEM_LOG_FILE_VAR				PMEM_LOG_FILE			N	Common initialisation of logging\*
libpmemblk		PMEMBLK_LOG_LEVEL_VAR			PMEMBLK_LOG_LEVEL		N	Common initialisation of logging\*
libpmemblk		PMEMBLK_LOG_FILE_VAR			PMEMBLK_LOG_FILE		N	Common initialisation of logging\*
libpmemlog		PMEMLOG_LOG_LEVEL_VAR			PMEMLOG_LOG_LEVEL		N	Common initialisation of logging\*
libpmemlog		PMEMLOG_LOG_FILE_VAR			PMEMLOG_LOG_FILE		N	Common initialisation of logging\*
libpmemobj		OBJ_CONFIG_ENV_VARIABLE			x						Y	Selectively overridden by settings from a file pointed to by 'OBJ_CONFIG_FILE_ENV_VARIABLE'
libpmemobj		OBJ_CONFIG_FILE_ENV_VARIABLE	x						Y	Points to a file path
libpmemobj										PMEMOBJ_COW				N	Only if library (not headers) compiled with -DUSE_COW_ENV (not default)
libpmemobj										PMEMOBJ_VG_CHECK_UNDEF	Y	Only if library (not headers) compiled with -DUSE_VG_MEMCHECK (not default)
libpmemobj		PMEMOBJ_LOG_LEVEL_VAR			PMEMOBJ_LOG_LEVEL		N	Common initialisation of logging\*
libpmemobj		PMEMOBJ_LOG_FILE_VAR			PMEMOBJ_LOG_FILE		N	Common initialisation of logging\*
libpmempool		PMEMPOOL_LOG_LEVEL_VAR			PMEMPOOL_LOG_LEVEL		N	Common initialisation of logging\*
libpmempool		PMEMPOOL_LOG_FILE_VAR			PMEMPOOL_LOG_FILE		N	Common initialisation of logging\*
librpmem		RPMEM_PROV_SOCKET_ENV			RPMEM_ENABLE_SOCKETS	?	Values must be 1 or 0; used as an int boolean. Default is 0. Enables libfabric sockets provider if available.
librpmem		RPMEM_PROV_VERBS_ENV			RPMEM_ENABLE_VERBS		?	Values must be 1 or 0; used as an int boolean. Default is 1 (true). Enables libfabric verbs provider if available.
librpmem		RPMEM_SSH_ENV					RPMEM_SSH				?	?
librpmem		RPMEM_CMD_ENV					RPMEM_CMD				?	?
librpmem		RPMEM_LOG_LEVEL_VAR				RPMEM_LOG_LEVEL			N	Common initialisation of logging\*
librpmem		RPMEM_LOG_FILE_VAR				RPMEM_LOG_FILE			N	Common initialisation of logging\*
libvmem			VMEM_LOG_LEVEL_VAR				VMEM_LOG_LEVEL			N	Common initialisation of logging\*
libvmem			VMEM_LOG_FILE_VAR				VMEM_LOG_FILE			N	Common initialisation of logging\*
libvmmalloc		VMMALLOC_POOL_DIR_VAR			VMMALLOC_POOL_DIR		N
libvmmalloc		VMMALLOC_POOL_SIZE_VAR			VMMALLOC_POOL_SIZE		N
libvmmalloc		VMMALLOC_FORK_VAR				VMMALLOC_FORK			N
libvmmalloc		VMMALLOC_LOG_STATS_VAR			VMMALLOC_LOG_STATS		N
libvmmalloc		VMMALLOC_LOG_LEVEL_VAR			VMMALLOC_LOG_LEVEL		N	Common initialisation of logging\*
libvmmalloc		VMMALLOC_LOG_FILE_VAR			VMMALLOC_LOG_FILE		N	Common initialisation of logging\*
common											NVML_LOG_ALIGN			N	Common initialisation of logging\*
common											PMEM_MMAP_HINT			?	No current production use cases
rpmem_common									SSH_CONNECTION			?
```


#### Notes

* Found using a search for calls to `os_getenv()`
* 'X' - can we set this variable from `main()` after the process has started and still influence outcomes?
* '*' - common initialisation of logging is done by `out_init()` from `common_init()`


### For tools

```
Tool	#define		Name
rpmend				SSH_CONNECTION
rpmend				USER
rpmend	HOME_ENV	HOME
```


### Testing

There are environment variables with names starting `UNITTEST_` `TEST_`, `ARCH_FLAGS_` and `RPMEMD_LOG_` (`RPMEMD_LOG_FILE` and `RPMEMD_LOG_LEVEL`).


### Benchmarks

There is the environment variable `PMEMBENCH_DIR`.

