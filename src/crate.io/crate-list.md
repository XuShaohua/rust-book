
# Core crates
* `rand` 随机数生成器, 随机分布算法, 随机序列
* `clap` 处理命令行选项
* `ansi_term` 调整终端字符串颜色与样式
* `tar` 处理 tarball 文件
* `flate2` 处理 gz 等压缩格式
* `data_encoding` 提供了base64以及byte array 转十六进制字符串的功能, 但是没有
  提供对于数据流的处理
* `base64` 支持 base32/base64 编码
* `ring` rust 语言实现的加密算法集合, 有很大部分代码是由汇编 和 C 实现的, 比如
  sha256/sha512 算法.
* `rusqlite` sqlite C 接口的绑定
* `postgres` postgresql 数据库的接口实现
* `error_chain` 处理 `Result<>` 更容易, 可以合并各种 `error` 类型.
* `regex` 正则表达式的实现, 实始化 `Pattern` 时性能不好
* `lazy_static` thread-local 数据, 第一次使用时初始化, 比如用于缓存
  `regex pattern` 数值
* `unicode_segmentation` 处理 UTF-8 转 Unicde 字符串
* `num_cpus` 类似于 `nproc` 命令, 返回系统中的 cpu 数量
* `chrono` 时间及日期处理
* `log`, `env_logger`, `syslog`, `log4rs` 都是日志模块
* `semver` 解析和处理版本号
* `reqwest` 类似于 python 里的 `requests` 模块, 用于处理客户端 HTTP 请求
* `memmap` 对 `mmap` 系统接口的绑定
* `walkdir` 遍历目录
* `glob` 目录文件名遍历匹配
* `url` URL 格式转换
* `percent_encoding` 将URL转为带百分号的转义字串
* `serde`, `serde_json`, `serde_derive` 提供了很方便的序列化/反序列化工具
* `threadpool` 线程池
* `rayon` 轻量级线程调度工具, 方便实现并发任务
* `url` 处理 URL
* `mime` 解析 MIME 类型
* `ansi_term` 终端中显示带颜色的文字

- byteorder
- lazy static
- rayon

- env_logger

## TODO
- actix
- actix-web
- actix_derive
- adler32
- aho-corasick
- alloc
- alloc_jemalloc
- alloc_system
- ammonia
- ansi_term
- app_dirs
- arena
- arrayvec
- aster
- atk-sys
- atty
- backtrace
- backtrace-sys
- base-x
- base32
- base64
- bincode
- bit-set
- bit-vec
- bitflags
- blooms-db
- bn
- brotli-sys
- brotli2
- build_const
- build_helper
- bytecount
- byteorder
- bytes
- c_vec
- cairo-rs
- cairo-sys-rs
- cargo_metadata
- cc
- cfg-if
- chalk-engine
- chalk-macros
- chrono
- cid
- clap
- cmake
- common-types
- compiler_builtins
- conv
- cookie
- core
- crc
- crossbeam
- crossbeam-channel
- crossbeam-deque
- crossbeam-epoch
- crossbeam-utils
- crunchy
- ct-logs
- ctrlc
- custom_derive
- daemonize
- datafrog
- dbghelp-sys
- derive-error-chain
- diesel
- diesel_cli
- diesel_derives
- dir
- docopt
- dotenv
- dtoa
- edit-distance
- either
- elastic-array
- elasticlunr-rs
- ena
- encoding
- encoding-index-japanese
- encoding-index-korean
- encoding-index-simpchinese
- encoding-index-singlebyte
- encoding-index-tradchinese
- encoding_index_tests
- error-chain
- eth-secp256k1
- ethabi
- ethabi-contract
- ethabi-derive
- ethash
- ethbloom
- ethcore
- ethcore-bloom-journal
- ethcore-bytes
- ethcore-crypto
- ethcore-devtools
- ethcore-io
- ethcore-light
- ethcore-logger
- ethcore-miner
- ethcore-network
- ethcore-network-devp2p
- ethcore-private-tx
- ethcore-service
- ethcore-stratum
- ethcore-sync
- ethcore-transaction
- ethereum-types
- ethereum-types-serialize
- ethjson
- ethkey
- ethstore
- evm
- extprim
- failure
- failure_derive
- fdlimit
- fetch
- filetime
- fixed-hash
- fixedbitset
- flate2
- fmt_macros
- fnv
- fs-swap
- futf
- futures
- futures-cpupool
- futures-timer
- gcc
- gdk
- gdk-pixbuf
- gdk-pixbuf-sys
- gdk-sys
- getopts
- gio
- gio-sys
- glib
- glib-sys
- glob
- globset
- gobject-sys
- graphviz
- gtk
- gtk-sys
- h2
- hamming
- handlebars
- hardware-wallet
- hashdb
- heapsize
- heck
- hex
- hidapi
- hostname
- html5ever
- http
- http-range
- httparse
- humantime
- hyper
- hyper-rustls
- idna
- igd
- indexmap
- infer_schema_internals
- inotify
- inotify-sys
- integer-encoding
- interleaved-ordered
- iovec
- ipnetwork
- iron
- is-match
- itertools
- itoa
- jobserver
- journaldb
- jsonrpc-core
- jsonrpc-http-server
- jsonrpc-ipc-server
- jsonrpc-macros
- jsonrpc-pubsub
- jsonrpc-server-utils
- jsonrpc-tcp-server
- jsonrpc-ws-server
- keccak-hash
- kernel32-sys
- kvdb
- kvdb-memorydb
- kvdb-rocksdb
- language-tags
- lazy_static
- lazycell
- libc
- libsqlite3-sys
- libusb
- libusb-sys
- linked-hash-map
- local-encoding
- log_settings
- lru-cache
- mac_address
- macros
- maplit
- markup5ever
- matches
- mdbook
- mem
- memchr
- memmap
- memoffset
- memory-cache
- memory_units
- memorydb
- migration-rocksdb
- migrations_internals
- mime
- mime_guess
- minifier
- miniz-sys
- miniz_oxide
- miniz_oxide_c_api
- mio
- mio-named-pipes
- mio-uds
- miow
- modifier
- mount
- msdos_time
- multibase
- multihash
- mysqlclient-sys
- nan-preserving-float
- net2
- new_debug_unreachable
- nix
- node-filter
- node-health
- nodrop
- notify
- ntp
- num
- num-bigint
- num-integer
- num-iter
- num-traits
- num_cpus
- number_prefix
- onig_sys
- open
- order-stat
- ordered-float
- ordermap
- owning_ref
- pango
- pango-sys
- panic_abort
- panic_hook
- panic_unwind
- parity-dapps
- parity-dapps-glue
- parity-hash-fetch
- parity-ipfs-api
- parity-local-store
- parity-machine
- parity-reactor
- parity-rpc
- parity-rpc-client
- parity-tokio-ipc
- parity-updater
- parity-version
- parity-wasm
- parity-whisper
- parity-wordlist
- parking_lot
- parking_lot_core
- path
- patricia-trie
- percent-encoding
- pest
- pest_derive
- petgraph
- phf
- phf_codegen
- phf_generator
- phf_shared
- pkg-config
- plain_hasher
- plist
- plugin
- podio
- polonius-engine
- pq-sys
- precomputed-hash
- pretty_env_logger
- price-info
- primal
- primal-bit
- primal-check
- primal-estimate
- primal-sieve
- proc-macro2
- proc-macro
- protobuf
- pulldown-cmark
- pwasm-utils
- quasi
- quasi_codegen
- quick-error
- quote
- rand
- rand_core
- rayon
- rayon-core
- regex
- regex-syntax
- registrar
- relay
- remove_dir_all
- resolv-conf
- ring
- rlp
- rlp_compress
- rlp_derive
- rls-data
- rls-span
- rocksdb
- rocksdb-sys
- rpassword
- rpc-cli
- rprompt
- rust-crypto
- rustc
- rustc-demangle
- rustc-hash
- rustc-hex
- rustc-main
- rustc-rayon
- rustc-rayon-core
- rustc-serialize
- rustc_allocator
- rustc_apfloat
- rustc_asan
- rustc_borrowck
- rustc_codegen_llvm
- rustc_codegen_utils
- rustc_cratesio_shim
- rustc_data_structures
- rustc_driver
- rustc_errors
- rustc_incremental
- rustc_lint
- rustc_llvm
- rustc_lsan
- rustc_metadata
- rustc_mir
- rustc_msan
- rustc_passes
- rustc_platform_intrinsics
- rustc_plugin
- rustc_privacy
- rustc_resolve
- rustc_save_analysis
- rustc_target
- rustc_traits
- rustc_tsan
- rustc_typeck
- rustc_version
- rustdoc
- rustdoc-tool
- rustls
- rustsym
- safemem
- same-file
- scoped-tls
- scopeguard
- sct
- semver
- semver-parser
- sequence_trie
- serde
- serde_derive
- serde_derive_internals
- serde_json
- serde_urlencoded
- serialize
- sha1
- shlex
- siphasher
- skeptic
- slab
- smallvec
- snappy
- snappy-sys
- stable_deref_trait
- staticfile
- stats
- std
- std_unicode
- stop-guard
- string
- string_cache
- string_cache_codegen
- string_cache_shared
- strsim
- strum
- strum_macros
- sval
- syn
- syncbox
- synom
- synstructure
- syntax
- syntax_ext
- syntax_pos
- syntex
- syntex_errors
- syntex_pos
- syntex_syntax
- take
- target_info
- tempdir
- tempfile
- tendril
- term
- term_size
- termcolor
- test
- textwrap
- thread-id
- thread_local
- time
- timer
- tiny-keccak
- tokio
- tokio-codec
- tokio-core
- tokio-current-thread
- tokio-executor
- tokio-fs
- tokio-io
- tokio-named-pipes
- tokio-proto
- tokio-reactor
- tokio-retry
- tokio-rustls
- tokio-service
- tokio-signal
- tokio-tcp
- tokio-threadpool
- tokio-timer
- tokio-udp
- tokio-uds
- toml
- toml-query
- trace-time
- traitobject
- transaction-pool
- transient-hashmap
- trezor-sys
- triehash
- trust-dns-proto
- trust-dns-resolver
- typeable
- typemap
- ucd-util
- uint
- unexpected
- unicase
- unicode-bidi
- unicode-normalization
- unicode-segmentation
- unicode-width
- unicode-xid
- unreachable
- unsafe-any
- untrusted
- unwind
- url
- using_queue
- utf-8
- utf8-ranges
- util-error
- uuid
- value-bag
- vec_map
- vergen
- version_check
- vm
- void
- walkdir
- wasm
- wasmi
- webpki
- webpki-roots
- winapi
- winapi-build
- ws
- ws2_32-sys
- xdg
- xml-rs
- xmltree
- zip

## DONE
- log
