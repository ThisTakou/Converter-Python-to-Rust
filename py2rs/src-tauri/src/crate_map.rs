use std::collections::HashMap;

pub struct LibInfo {
    pub crate_hint: &'static str,
    pub doc: &'static str,
}

macro_rules! lib {
    ($py:expr, $hint:expr, $doc:expr) => {
        ($py, LibInfo { crate_hint: $hint, doc: $doc })
    };
}

/// Расширенный каталог 300+ популярных модулей Python с готовой документацией для ИИ,
/// чтобы агент не переспрашивал пользователя. Для остального пользователь пишет документацию
/// сам в MD-окне в интерфейсе.
pub fn catalog() -> HashMap<&'static str, LibInfo> {
    [
        // --- уже проверенные версией без документации (сохранены как были) ---
        lib!("requests", "reqwest", "Simple HTTP client for GET/POST calls and REST APIs. reqwest is async by default; reqwest::blocking gives a synchronous style closer to requests."),
        lib!("httpx", "reqwest", "Async/sync HTTP client, same role as requests. Use reqwest with tokio for the async parts."),
        lib!("urllib", "reqwest", "Standard-library URL/HTTP handling. Use reqwest for requests and the url crate for URL parsing."),
        lib!("flask", "axum", "Lightweight web framework for routes and handlers. axum (or actix-web) gives a similarly small router on top of tokio."),
        lib!("fastapi", "axum", "Async web framework with typed request/response models. axum plus serde for the models is the closest match."),
        lib!("django", "actix-web", "Full-stack framework with ORM, admin and templating built in. No single crate matches all of it: actix-web plus sea-orm or diesel plus askama cover routing, database and templates separately."),
        lib!("aiohttp", "reqwest + tokio", "Async HTTP client and server. reqwest covers the client side; axum or actix-web covers the server side, both on tokio."),
        lib!("numpy", "ndarray", "N-dimensional arrays and vectorised math. ndarray gives similar array types and operations such as mapv, slicing and broadcasting."),
        lib!("pandas", "polars", "DataFrames for tabular data analysis. polars offers a similar DataFrame API with lazy evaluation and is generally faster."),
        lib!("scipy", "nalgebra", "Scientific computing: linear algebra, optimisation, stats. nalgebra covers linear algebra; other areas need separate crates such as argmin or statrs."),
        lib!("sklearn", "linfa", "Classical ML algorithms such as regression and clustering. linfa provides similar estimators with a fit/predict style API."),
        lib!("torch", "tch (libtorch bindings)", "Deep learning with tensors and autograd. tch binds to libtorch, the same C++ backend PyTorch itself uses."),
        lib!("matplotlib", "plotters", "2D plotting. plotters draws charts to PNG, SVG or a GUI backend with a builder-style API."),
        lib!("sqlalchemy", "sqlx или diesel", "SQL toolkit and ORM. sqlx gives async, compile-time-checked raw SQL; diesel gives a more traditional compile-time-checked ORM."),
        lib!("psycopg2", "sqlx (postgres feature)", "PostgreSQL driver. sqlx's postgres feature, or tokio-postgres directly, covers connecting and querying."),
        lib!("pymysql", "sqlx (mysql feature)", "MySQL driver. sqlx's mysql feature covers the same connection and query operations."),
        lib!("sqlite3", "rusqlite", "Bundled SQLite database. rusqlite wraps the same SQLite C library with a similar connection and query API."),
        lib!("redis", "redis", "Redis client. The redis crate exposes the same commands such as get, set and pub-sub, with sync and async clients."),
        lib!("pydantic", "serde", "Typed data models with validation. serde derives (de)serialization for structs; add the validator crate for field-level rules."),
        lib!("json", "serde_json", "Parse and produce JSON. serde_json (de)serializes to and from serde-derived structs or a generic Value type."),
        lib!("yaml", "serde_yaml", "Parse and produce YAML. serde_yaml works the same way as serde_json but for YAML."),
        lib!("toml", "toml", "Parse and produce TOML, also Rust's own config format. The toml crate (de)serializes via serde just like serde_json."),
        lib!("re", "regex", "Regular expressions. The regex crate implements the same style of pattern matching, compiled ahead of time for speed."),
        lib!("datetime", "chrono", "Dates, times and arithmetic on them. chrono provides equivalent DateTime and NaiveDate types with the same kind of arithmetic."),
        lib!("time", "std::time / chrono", "Low-level time access such as timestamps and sleep. std::time covers Instant, SystemTime and Duration; chrono covers calendar dates."),
        lib!("os", "std::env / std::fs", "OS interface: environment variables, filesystem, process info. std::env covers vars and args, std::fs covers file operations."),
        lib!("pathlib", "std::path::PathBuf", "Object-oriented filesystem paths. std::path::Path and PathBuf give the same join, parent and extension style API."),
        lib!("subprocess", "std::process::Command", "Run external commands and capture their output. std::process::Command builds and spawns a child process the same way."),
        lib!("threading", "std::thread", "OS threads for concurrency. std::thread::spawn plus channels or Mutex and Arc cover the same patterns."),
        lib!("multiprocessing", "std::thread / rayon", "Separate processes to bypass the GIL for CPU-bound work. Since Rust has no GIL, std::thread or rayon's parallel iterators usually replace this directly."),
        lib!("asyncio", "tokio", "Async event loop, tasks and coroutines. tokio is the standard async runtime; async fn plus .await replaces async def and await."),
        lib!("logging", "tracing или log + env_logger", "Structured logging with levels. tracing adds spans plus levels; log with env_logger is the simpler, more direct equivalent."),
        lib!("argparse", "clap", "Command-line argument parsing. clap builds a parser, including --help text, from a struct via derive macros."),
        lib!("click", "clap", "Higher-level CLI framework with decorators for commands and options. clap's derive API covers subcommands and options the same way."),
        lib!("pytest", "#[test] встроенные тесты", "Test framework with fixtures and parametrisation. Rust's built-in #[test] functions cover most of the same ground; the rstest crate adds fixtures and parametrised tests."),
        lib!("unittest", "#[test] встроенные тесты", "Standard-library test framework based on TestCase classes. Rust's built-in #[test] functions replace this directly, no framework needed."),
        lib!("itertools", "itertools", "Extra iterator helpers such as chain and groupby. The itertools crate adds the same combinators on top of Rust's own Iterator trait."),
        lib!("collections", "std::collections", "Extra container types such as deque, Counter and defaultdict. std::collections has direct equivalents: VecDeque, HashMap with its entry API, and so on."),
        lib!("dataclasses", "struct + #[derive]", "Simple data-holding classes. A plain struct with #[derive(Debug, Clone, PartialEq)] covers the same use case."),
        lib!("enum", "enum (встроенный)", "Enumerated constant types. Rust's built-in enum keyword covers this directly, with the added benefit of carrying data per variant."),
        lib!("random", "rand", "Random number generation. The rand crate covers the same range of distributions and shuffling helpers."),
        lib!("hashlib", "sha2 / md-5", "Cryptographic hash functions such as SHA-256 and MD5. Pick the matching crate per algorithm, for example sha2 for SHA-256."),
        lib!("base64", "base64", "Base64 encode and decode. The base64 crate provides the same encode and decode functions."),
        lib!("uuid", "uuid", "Generate and parse UUIDs. The uuid crate covers the same v4 random and other UUID versions."),
        lib!("boto3", "aws-sdk-rust", "AWS SDK for all services such as S3 and DynamoDB. The official aws-sdk-rust crates, one per service, mirror the same clients and calls."),
        lib!("jinja2", "askama или tera", "Template rendering with double-brace syntax. askama compiles templates at build time and is type-checked; tera renders them at runtime, closer to Jinja2's own style."),
        lib!("bs4", "scraper", "Parse HTML and query it with selectors. scraper does the same using CSS selectors over a parsed DOM."),
        lib!("lxml", "roxmltree", "Fast XML and HTML parsing backed by C. roxmltree offers a similarly fast read-only DOM for XML."),
        lib!("pillow", "image", "Open, edit and save images. The image crate covers the same decode, resize and encode operations."),
        lib!("pil", "image", "Older name for Pillow, same image-handling role. The image crate is the same equivalent."),
        lib!("cryptography", "ring или rustls", "General-purpose crypto primitives such as ciphers and signatures. ring covers primitives directly; rustls builds TLS on top of similar primitives."),
        lib!("jwt", "jsonwebtoken", "Encode and verify JSON Web Tokens. The jsonwebtoken crate signs and verifies tokens the same way."),
        lib!("websockets", "tokio-tungstenite", "Async WebSocket client and server. tokio-tungstenite provides the same primitives on top of tokio."),
        lib!("celery", "нет прямого аналога: очередь задач, например через lapin (RabbitMQ)", "Distributed task queue. There is no single equivalent; lapin for RabbitMQ, or a Redis-based queue plus your own worker loop, replaces it."),

        // --- сеть ---
        lib!("socket", "std::net", "Low-level TCP and UDP sockets. std::net covers TcpStream, TcpListener and UdpSocket synchronously; tokio::net gives the async versions."),
        lib!("ssl", "rustls или native-tls", "TLS for network connections. rustls, pure Rust, or native-tls using the OS TLS stack, wrap a std::net or tokio::net stream."),
        lib!("ftplib", "suppaftp", "FTP client. The suppaftp crate is the closest maintained equivalent."),
        lib!("smtplib", "lettre", "Send email over SMTP. lettre provides a typed builder for messages and transports."),
        lib!("email", "lettre", "Build and parse email messages, including MIME and headers. lettre covers building and sending; mail-parser covers parsing."),
        lib!("http", "http (crate) / hyper", "Shared HTTP types such as status codes and headers. The http crate defines the types used by hyper, reqwest and axum."),
        lib!("select", "tokio", "Wait on multiple I/O sources at once. tokio's async runtime replaces most select or poll-style code."),
        lib!("paramiko", "russh", "SSH client for remote commands and SFTP. russh is a pure-Rust SSH implementation; ssh2 wraps libssh2 as an alternative."),

        // --- веб-фреймворки ---
        lib!("starlette", "axum", "ASGI toolkit that FastAPI is built on. axum's tower-based middleware stack plays the same role."),
        lib!("uvicorn", "n/a — сервер уже встроен в axum/actix-web", "ASGI server that runs async Python apps. A compiled axum or actix-web binary already includes its own async server, so no separate runner is needed."),
        lib!("gunicorn", "n/a — параллелизм уже даёт tokio/actix", "WSGI process manager for Python web apps. A compiled Rust web server already handles concurrency itself via tokio or actix, so no separate process manager is typically needed."),

        // --- данные / числа / ML ---
        lib!("tensorflow", "tensorflow (rust bindings) or burn", "Deep learning framework. Rust has bindings to the TensorFlow C API, or the pure-Rust burn framework as an alternative."),
        lib!("seaborn", "plotters", "Statistical plotting on top of matplotlib. Build the same chart types directly with plotters; there is no direct one-to-one crate."),
        lib!("plotly", "plotters или charming", "Interactive charts. plotters covers static charts; charming produces interactive ECharts-style HTML charts."),
        lib!("xgboost", "xgboost (rust bindings)", "Gradient-boosted trees. The xgboost crate wraps the same underlying C++ library."),
        lib!("lightgbm", "lightgbm (rust bindings)", "Gradient-boosted trees tuned for large tabular data. The lightgbm crate wraps the official C library."),
        lib!("networkx", "petgraph", "Graph data structures and algorithms. petgraph provides graph types plus common algorithms such as shortest path and traversal."),
        lib!("statistics", "statrs", "Basic statistics such as mean and standard deviation. Simple ones can be written by hand; statrs adds distributions and more advanced stats."),
        lib!("decimal", "rust_decimal", "Fixed-point decimal arithmetic for money-like values. rust_decimal avoids the same floating-point rounding issues."),
        lib!("fractions", "num-rational", "Exact rational numbers. num-rational's Ratio type supports the same arithmetic."),
        lib!("array", "Vec<T>", "Compact typed arrays. Rust's Vec<T>, or a fixed-size array, covers this directly with no crate needed."),
        lib!("numba", "n/a — код на Rust уже скомпилирован", "JIT-compiles Python functions for speed. Rust code is already compiled, so this concern does not carry over; focus on efficient algorithms and data structures instead."),
        lib!("joblib", "rayon", "Parallel loops and simple caching. rayon's parallel iterators give easy data-parallelism; caching needs to be written by hand or via the cached crate."),
        lib!("dask", "polars", "Parallel and out-of-core dataframes. polars already streams larger-than-memory data itself; rayon adds parallelism to plain compute."),

        // --- базы данных ---
        lib!("pymongo", "mongodb", "MongoDB driver. The official mongodb crate mirrors the same client, collection and document API."),

        // --- конфиги / сериализация ---
        lib!("configparser", "ini", "Read INI-style config files. The ini crate parses the same format directly."),
        lib!("pickle", "serde + bincode", "Serialize arbitrary objects to bytes. There is no byte-compatible equivalent; use serde with bincode, or serde_json, for Rust-to-Rust (de)serialization instead."),
        lib!("marshmallow", "serde", "Schema-based (de)serialization and validation, similar role to pydantic. serde plus the validator crate covers both parts."),
        lib!("attrs", "struct + #[derive]", "Boilerplate-free classes with generated methods. A struct with #[derive(Debug, Clone)] already generates the equivalent methods."),

        // --- CLI / логи / инструменты разработки ---
        lib!("typer", "clap", "Type-hint-based CLI framework built on click. clap's derive API, generated from a struct, plays the same role."),
        lib!("loguru", "tracing", "Batteries-included logging with nice formatting out of the box. tracing plus tracing-subscriber gets close, with a bit more setup."),
        lib!("rich", "owo-colors", "Pretty terminal output such as colours and tables. owo-colors handles colouring text; indicatif, see tqdm, handles progress bars."),
        lib!("tqdm", "indicatif", "Progress bars for loops. indicatif's ProgressBar wraps an iterator or a manual counter the same way."),
        lib!("black", "rustfmt", "Opinionated code formatter. rustfmt is the standard formatter, run via cargo fmt, with a similar no-config philosophy."),
        lib!("flake8", "clippy", "Linter for style and common mistakes. cargo clippy is the standard Rust linter, run alongside rustfmt."),
        lib!("mypy", "n/a — типы уже проверяет компилятор", "Static type checker for optionally-typed Python. Rust's compiler already enforces types on every build, so no separate tool is needed."),
        lib!("pip", "cargo", "Python's package installer. cargo is Rust's package manager and build tool combined, for example cargo add and cargo build."),
        lib!("setuptools", "cargo", "Build and packaging configuration for Python projects. Cargo.toml plus cargo build or cargo publish covers packaging and building."),
        lib!("virtualenv", "n/a — Cargo уже изолирует зависимости", "Isolated Python environments per project. Cargo already isolates dependencies per project via Cargo.lock, no separate tool is needed."),
        lib!("poetry", "cargo", "Dependency management and packaging. Cargo already combines both roles natively."),

        // --- параллелизм / система ---
        lib!("signal", "signal-hook", "Handle OS signals such as SIGINT and SIGTERM. signal-hook, or tokio::signal for async code, registers handlers the same way."),
        lib!("ctypes", "libc", "Call into C libraries via FFI. Rust's own extern blocks plus the libc crate for common types cover the same FFI calls."),
        lib!("platform", "std::env", "Query OS and platform info. std::env::consts covers OS and architecture; the sys-info crate adds more detailed system info."),
        lib!("getpass", "rpassword", "Read a password from the terminal without echoing it. The rpassword crate provides the same hidden-input prompt."),
        lib!("gc", "n/a — в Rust нет сборщика мусора", "Manual garbage-collector control. Rust uses ownership and borrowing instead of a garbage collector, so this module has no equivalent to configure."),
        lib!("weakref", "std::rc::Weak", "Non-owning references that do not keep an object alive. std::rc::Weak, or std::sync::Weak across threads, is the direct equivalent."),

        // --- файлы / пути / форматы ---
        lib!("shutil", "std::fs", "Higher-level file operations such as copy, move and rmtree. std::fs::copy, rename and remove_dir_all cover the same operations."),
        lib!("glob", "glob", "Match files by wildcard pattern. The glob crate provides the same pattern matching over the filesystem."),
        lib!("tempfile", "tempfile", "Create temporary files and directories that clean up automatically. The tempfile crate does the same, tied to Rust's drop semantics."),
        lib!("csv", "csv", "Read and write CSV files. The csv crate reads and writes rows, and pairs well with serde for typed rows."),
        lib!("zipfile", "zip", "Read and write ZIP archives. The zip crate covers the same reading and writing of entries."),
        lib!("tarfile", "tar", "Read and write tar archives. The tar crate covers the same operations, often paired with flate2 for tar.gz files."),
        lib!("gzip", "flate2", "Gzip compression and decompression. flate2 wraps the same underlying zlib and gzip algorithms."),
        lib!("zlib", "flate2", "Raw deflate and zlib compression. flate2 covers this directly, the same underlying algorithm."),

        // --- текст / парсинг ---
        lib!("regex", "regex", "Third-party regex engine with extra features beyond the standard re module. Rust's regex crate covers most common cases; fancy-regex adds lookaround."),
        lib!("textwrap", "textwrap", "Wrap, fill or indent text to a given width. The textwrap crate provides the same wrapping functions."),
        lib!("difflib", "similar", "Compute diffs between sequences or text. The similar crate computes comparable diffs and can render them."),
        lib!("pprint", "Debug derive", "Pretty-print nested data structures. Deriving Debug and using the alternate {:#?} format gives a similarly readable layout."),
        lib!("html", "html-escape", "Parse or escape HTML. scraper parses and queries HTML with CSS selectors; html-escape handles entity escaping."),
        lib!("xml", "quick-xml", "Parse and generate XML. quick-xml is a fast streaming parser; roxmltree is a simpler read-only alternative."),
        lib!("faker", "fake", "Generate realistic fake data for tests or seeding. The fake crate generates similar random names, addresses and emails."),

        // --- безопасность / кодирование ---
        lib!("hmac", "hmac", "Keyed-hash message authentication codes. The hmac crate implements the same construction over a chosen hash function."),
        lib!("secrets", "rand (with OsRng)", "Cryptographically secure random values such as tokens and passwords. rand::rngs::OsRng is the equivalent secure source."),

        // --- коллекции / функциональщина ---
        lib!("heapq", "std::collections::BinaryHeap", "Heap and priority-queue operations on a list. BinaryHeap provides the same push and pop-largest behaviour; wrap values in Reverse for a min-heap."),
        lib!("bisect", "slice::binary_search", "Binary search and sorted insertion into a list. Rust's slice::binary_search and binary_search_by cover the same operations."),
        lib!("queue", "std::sync::mpsc", "Thread-safe queues for producer and consumer patterns. std::sync::mpsc channels, or crossbeam-channel for more features, replace this directly."),
        lib!("operator", "std operator traits", "Operators exposed as functions, such as add or itemgetter. Rust's operator traits like Add and Index, plus closures, cover the same use cases directly."),

        // --- время ---
        lib!("pytz", "chrono-tz", "Timezone database for datetime objects. chrono-tz provides the same IANA timezone database for use with chrono."),
        lib!("dateutil", "chrono", "Extra date parsing and arithmetic helpers on top of datetime. chrono's parsing functions and Duration arithmetic cover most of the same cases."),

        // --- облако / инфраструктура ---
        lib!("docker", "bollard", "Talk to the Docker daemon API. The bollard crate wraps the same Docker Engine API."),
        lib!("grpc", "tonic", "gRPC client and server. tonic is the standard async gRPC implementation, built on the same protobuf definitions."),
        lib!("protobuf", "prost", "Protocol Buffers (de)serialization. prost generates the same kind of typed structs from .proto files."),
        lib!("msgpack", "rmp-serde", "Compact binary (de)serialization. rmp-serde (de)serializes MessagePack via serde, just like serde_json does for JSON."),
        lib!("ujson", "serde_json", "Faster JSON library, same role as the standard json module. serde_json is already fast, so no separate crate is needed."),
        lib!("orjson", "serde_json", "Fast JSON library with stricter output. serde_json covers the same (de)serialization needs."),
        lib!("cachetools", "lru", "In-memory caches with eviction policies such as LRU or TTL. The lru crate gives an LRU cache; the cached crate adds macro-based memoisation."),
        lib!("tenacity", "backoff", "Retry logic with backoff for flaky calls. The backoff crate implements the same retry and backoff strategies."),

        // --- GUI / игры / зрение ---
        lib!("pygame", "macroquad", "2D game framework covering rendering, input and sound. macroquad or ggez provide a similarly simple 2D game loop and drawing API."),
        lib!("kivy", "egui или iced", "Cross-platform GUI framework. egui or iced are the closest idiomatic Rust GUI toolkits."),
        lib!("cv2", "opencv", "Computer vision via OpenCV bindings. The opencv crate wraps the same underlying OpenCV C++ library."),
        lib!("selenium", "thirtyfour", "Browser automation via WebDriver. thirtyfour talks to the same WebDriver protocol, for example chromedriver."),
        lib!("scrapy", "reqwest + scraper", "Web-scraping framework combining crawling and parsing. Combine reqwest for fetching pages with scraper for parsing them; there is no single all-in-one crate."),
        lib!("nltk", "rust-bert", "Natural-language processing toolkit for tokenising and tagging. rust-bert covers modern transformer-based NLP; simple tokenising can be done by hand."),
        lib!("spacy", "rust-bert", "Production NLP pipelines such as NER and parsing. rust-bert provides similar pretrained-model pipelines via Hugging Face format models."),
        lib!("gym", "n/a — обычно пишется вручную под проект", "Reinforcement-learning environment interface. There is no widely standard Rust equivalent; environments are typically hand-written per project."),

        // --- дополнительные веб-фреймворки ---
        lib!("tornado", "actix-web", "Async web framework with non-blocking I/O. actix-web provides the same async handlers and routing."),
        lib!("sanic", "actix-web", "Fast async web framework. actix-web is similarly fast and async-first."),
        lib!("bottle", "warp", "Minimal web microframework for simple APIs. warp provides a filter-based routing system for small services."),
        lib!("cherrypy", "actix-web", "Object-oriented HTTP framework. actix-web with extractors and handlers covers the same ground."),
        lib!("pyramid", "actix-web", "Flexible web framework for complex apps. actix-web plus separate crates for ORM and templates match the modularity."),

        // --- дополнительные HTTP/сетевые библиотеки ---
        lib!("urllib3", "reqwest", "Low-level HTTP client library that requests uses internally. reqwest already wraps hyper; for lower-level work use hyper directly."),
        lib!("httplib2", "reqwest", "HTTP client with caching support. reqwest handles the requests; add http-cache-semantics or tower-http middleware for caching."),
        lib!("pycurl", "reqwest или curl (rust bindings)", "libcurl bindings for advanced HTTP. reqwest covers most needs; the curl crate wraps libcurl directly if needed."),
        lib!("twisted", "tokio", "Event-driven networking engine for protocols beyond HTTP. tokio provides the same async primitives for building custom protocols."),
        lib!("socketio", "rust-socketio", "Socket.IO client for real-time communication. rust-socketio implements the Socket.IO protocol client-side."),
        lib!("zeromq", "zmq", "High-performance async messaging library. The zmq crate wraps the same underlying ZeroMQ C library."),
        lib!("kafka-python", "rdkafka", "Apache Kafka client. rdkafka wraps librdkafka and provides async producers and consumers."),
        lib!("pika", "lapin", "RabbitMQ AMQP client. lapin is an async pure-Rust AMQP 0.9.1 client for RabbitMQ."),
        lib!("grpcio", "tonic", "gRPC library based on C core. tonic is a pure-Rust async gRPC implementation."),

        // --- дополнительные базы данных ---
        lib!("cassandra-driver", "cassandra-cpp", "Apache Cassandra client. cassandra-cpp wraps the DataStax C++ driver."),
        lib!("elasticsearch", "elasticsearch", "Elasticsearch client. The official elasticsearch crate covers the REST API."),
        lib!("neo4j", "neo4rs", "Neo4j graph database client. neo4rs provides async Bolt protocol support."),
        lib!("influxdb", "influxdb", "InfluxDB time-series database client. The influxdb crate wraps the HTTP API."),
        lib!("clickhouse-driver", "clickhouse", "ClickHouse database client. The clickhouse crate supports both HTTP and native protocols."),
        lib!("motor", "mongodb", "Async MongoDB driver. The official mongodb crate already has async support via tokio."),
        lib!("aiomysql", "sqlx (mysql feature)", "Async MySQL driver. sqlx's mysql feature provides the same async operations."),
        lib!("aiopg", "sqlx (postgres feature)", "Async PostgreSQL driver. sqlx's postgres feature covers async queries."),
        lib!("asyncpg", "sqlx (postgres feature)", "Fast async PostgreSQL driver. sqlx already provides fast async PostgreSQL access."),

        // --- дополнительная сериализация ---
        lib!("avro", "apache-avro", "Apache Avro (de)serialization. apache-avro implements the same binary format with schema support."),
        lib!("cbor", "ciborium", "Concise Binary Object Representation. ciborium (de)serializes CBOR via serde."),
        lib!("cbor2", "ciborium", "CBOR implementation. Same as cbor, use ciborium."),
        lib!("bson", "bson", "BSON (de)serialization for MongoDB. The bson crate works with serde and mongodb."),
        lib!("pyyaml", "serde_yaml", "YAML parser and emitter. serde_yaml does the same (de)serialization."),
        lib!("ruamel.yaml", "serde_yaml", "Advanced YAML library preserving comments. serde_yaml handles the format; comment preservation requires custom handling."),
        lib!("xmltodict", "quick-xml + serde", "Convert XML to dictionary structures. Combine quick-xml with serde for similar struct mapping."),

        // --- дополнительная обработка данных ---
        lib!("openpyxl", "calamine или rust_xlsxwriter", "Read and write Excel files (.xlsx). calamine reads Excel files; rust_xlsxwriter writes them."),
        lib!("xlrd", "calamine", "Read older Excel files (.xls). calamine supports both .xls and .xlsx formats."),
        lib!("xlwt", "rust_xlsxwriter", "Write Excel files. rust_xlsxwriter creates .xlsx files."),
        lib!("tabulate", "prettytable-rs", "Pretty-print tabular data. prettytable-rs renders tables to the terminal."),
        lib!("arrow", "arrow", "Apache Arrow columnar data format. The official arrow crate implements the same in-memory format."),
        lib!("parquet", "parquet", "Apache Parquet columnar storage. The parquet crate reads and writes the same format."),
        lib!("h5py", "hdf5", "HDF5 file format for scientific data. The hdf5 crate wraps libhdf5."),
        lib!("netcdf4", "netcdf", "NetCDF file format. The netcdf crate wraps the C library."),

        // --- дополнительные тестовые инструменты ---
        lib!("mock", "mockall", "Mocking framework for unit tests. mockall generates mock objects via macros."),
        lib!("responses", "mockito", "Mock HTTP responses for tests. mockito creates mock HTTP servers."),
        lib!("faker", "fake", "Generate fake data. The fake crate produces random names, emails, addresses."),
        lib!("hypothesis", "proptest или quickcheck", "Property-based testing. proptest and quickcheck generate test cases automatically."),
        lib!("coverage", "cargo-tarpaulin", "Code coverage measurement. cargo-tarpaulin generates coverage reports."),
        lib!("pytest-asyncio", "#[tokio::test]", "Async test support for pytest. Use #[tokio::test] for async test functions."),
        lib!("nose", "#[test]", "Test runner. Rust's built-in cargo test replaces this."),

        // --- дополнительные CLI/TUI библиотеки ---
        lib!("prompt_toolkit", "dialoguer", "Interactive CLI prompts and completions. dialoguer provides input, confirmation, selection prompts."),
        lib!("questionary", "dialoguer", "User-friendly CLI prompts. dialoguer covers the same interactive prompt types."),
        lib!("blessed", "crossterm", "Terminal manipulation and colors. crossterm handles cursor movement, colors, events."),
        lib!("colorama", "colored", "ANSI color codes for terminal output. colored provides the same text coloring API."),
        lib!("termcolor", "termcolor", "Cross-platform terminal colors. The termcolor crate does the same."),
        lib!("asciimatics", "tui-rs (ratatui)", "Full-screen text UI animations. ratatui builds terminal UIs with widgets and layouts."),
        lib!("curses", "pancurses", "Terminal UI library. pancurses wraps ncurses/pdcurses cross-platform."),

        // --- дополнительная криптография ---
        lib!("nacl", "sodiumoxide", "Modern crypto library (libsodium bindings). sodiumoxide wraps libsodium with the same primitives."),
        lib!("pynacl", "sodiumoxide", "Python bindings to libsodium. sodiumoxide is the Rust equivalent."),
        lib!("passlib", "argon2 или bcrypt", "Password hashing library. Use argon2 for modern hashing or bcrypt for compatibility."),
        lib!("itsdangerous", "jsonwebtoken", "Sign and verify data, often for tokens. jsonwebtoken handles JWT; for general signing use ring or sodiumoxide."),
        lib!("fernet", "fernet", "Symmetric encryption for messages. The fernet crate implements the same spec."),
        lib!("oauthlib", "oauth2", "OAuth 1.0/2.0 client and server. The oauth2 crate handles OAuth 2.0 flows."),

        // --- дополнительная обработка текста ---
        lib!("markdown", "pulldown-cmark", "Parse Markdown to HTML. pulldown-cmark is a fast CommonMark parser."),
        lib!("mistune", "pulldown-cmark", "Markdown parser. pulldown-cmark covers the same use case."),
        lib!("pyparsing", "nom или pest", "Build recursive-descent parsers. nom provides parser combinators; pest uses PEG grammars."),
        lib!("parsimonious", "pest", "PEG parser generator. pest generates parsers from PEG grammar files."),
        lib!("chardet", "chardetng", "Character encoding detection. chardetng detects text encodings."),
        lib!("ftfy", "manual logic via String methods", "Fix mojibake and encoding errors. No direct crate; handle case-by-case with String methods."),
        lib!("unidecode", "deunicode", "Transliterate Unicode to ASCII. deunicode performs the same transliteration."),
        lib!("python-slugify", "slug", "Convert strings to URL-safe slugs. The slug crate does the same."),

        // --- дополнительная валидация ---
        lib!("cerberus", "validator", "Schema validation for data structures. The validator crate provides derive macros for field validation."),
        lib!("voluptuous", "validator", "Data validation library. Use validator with serde for schema-based validation."),
        lib!("schema", "serde + validator", "Declarative data validation. Combine serde for structure with validator for rules."),
        lib!("email-validator", "validator (with email feature)", "Email address validation. The validator crate includes email validation."),

        // --- дополнительные файлы/форматы ---
        lib!("magic", "infer", "Detect file type from content. infer identifies file types by magic bytes."),
        lib!("python-magic", "infer", "File type detection via libmagic. infer is a pure-Rust alternative."),
        lib!("chardet", "chardetng", "Encoding detection. chardetng detects character encodings."),
        lib!("bz2", "bzip2", "Bzip2 compression. The bzip2 crate wraps libbz2."),
        lib!("lzma", "xz2", "LZMA/XZ compression. xz2 wraps liblzma."),
        lib!("zstandard", "zstd", "Zstandard compression. The zstd crate wraps libzstd."),
        lib!("py7zr", "sevenz-rust", "7-Zip archive handling. sevenz-rust reads and writes .7z files."),

        // --- дополнительное время/планирование ---
        lib!("schedule", "tokio + tokio-cron-scheduler", "Job scheduling library. tokio-cron-scheduler runs periodic tasks."),
        lib!("apscheduler", "tokio + tokio-cron-scheduler", "Advanced task scheduler. Use tokio-cron-scheduler for cron-like scheduling."),
        lib!("croniter", "cron", "Parse cron expressions. The cron crate parses cron schedules."),
        lib!("arrow", "chrono", "Better dates and times. chrono is the standard date/time library."),
        lib!("pendulum", "chrono + chrono-tz", "Datetime library with timezone support. Combine chrono with chrono-tz."),

        // --- дополнительные системные утилиты ---
        lib!("psutil", "sysinfo", "System and process utilities. sysinfo provides CPU, memory, process info."),
        lib!("py-cpuinfo", "sysinfo", "CPU information. sysinfo includes CPU details."),
        lib!("distro", "std::env::consts", "Linux distribution info. std::env::consts::OS gives basic OS info; sys-info adds details."),
        lib!("watchdog", "notify", "Filesystem event monitoring. notify watches file changes across platforms."),
        lib!("sh", "std::process::Command", "Shell command execution. std::process::Command runs external commands."),
        lib!("delegator", "std::process::Command", "Subprocess management. std::process::Command handles this directly."),
        lib!("envparse", "envy", "Parse environment variables into structs. envy deserializes env vars via serde."),
        lib!("python-dotenv", "dotenv", "Load environment variables from .env files. The dotenv crate does the same."),

        // --- дополнительные дата-валидация ---
        lib!("phonenumbers", "phonenumber", "Parse and validate phone numbers. The phonenumber crate wraps libphonenumber."),
        lib!("iso8601", "chrono", "Parse ISO 8601 date/time strings. chrono::DateTime::parse_from_rfc3339 handles ISO 8601."),
        lib!("isodate", "chrono", "ISO 8601 date parsing. chrono covers this natively."),

        // --- дополнительные образы/медиа ---
        lib!("imageio", "image", "Read and write images and videos. The image crate handles images; for video use ffmpeg bindings."),
        lib!("opencv-python", "opencv", "OpenCV bindings. The opencv crate wraps the same C++ library."),
        lib!("scikit-image", "imageproc", "Image processing algorithms. imageproc provides filters, transformations, feature detection."),
        lib!("pdf2image", "pdfium-render", "Convert PDF pages to images. pdfium-render renders PDFs to image buffers."),
        lib!("pypdf", "lopdf", "Low-level PDF manipulation. lopdf reads and writes PDF structure directly."),
        lib!("reportlab", "printpdf", "Generate PDFs programmatically. printpdf creates PDFs from scratch."),

        // --- дополнительное аудио ---
        lib!("pydub", "rodio", "Audio manipulation and playback. rodio decodes and plays audio; for editing combine with hound or similar."),
        lib!("soundfile", "hound", "Read and write audio files. hound handles WAV files; for more formats use symphonia."),
        lib!("librosa", "n/a — audio DSP hand-coded or via rustfft", "Audio analysis library. No single equivalent; use rustfft for spectral analysis or write DSP by hand."),

        // --- дополнительные геоданные ---
        lib!("geopy", "geocoding", "Geocoding services. The geocoding crate talks to OpenStreetMap and other APIs."),
        lib!("shapely", "geo", "Geometric objects and operations. The geo crate provides points, lines, polygons and spatial operations."),
        lib!("geopandas", "geo + geojson", "Geospatial dataframes. Combine geo for geometry with geojson for (de)serialization; no single DataFrame equivalent."),
        lib!("folium", "n/a — generate HTML map via template", "Interactive maps via Leaflet.js. No direct crate; generate HTML with embedded Leaflet.js manually."),

        // --- дополнительный интернет/скрапинг ---
        lib!("feedparser", "feed-rs", "Parse RSS and Atom feeds. feed-rs parses both formats."),
        lib!("newspaper3k", "reqwest + scraper", "Article extraction from web pages. Combine reqwest for fetching with scraper and custom logic for article detection."),
        lib!("trafilatura", "reqwest + scraper", "Web scraping and text extraction. Use reqwest + scraper; no single all-in-one crate."),

        // --- дополнительные финансы ---
        lib!("yfinance", "yahoo_finance_api", "Download market data from Yahoo Finance. yahoo_finance_api queries the same endpoints."),
        lib!("pandas-datareader", "yahoo_finance_api или manual HTTP", "Financial data from various sources. Use specific crates per source or reqwest to fetch manually."),
        lib!("stripe", "stripe-rust (async-stripe)", "Stripe payment API client. async-stripe wraps the Stripe REST API."),

        // --- дополнительные шаблоны ---
        lib!("mako", "tera", "Template engine. tera provides Jinja2-like syntax with runtime rendering."),
        lib!("chameleon", "tera или askama", "XML-based template engine. tera or askama cover template rendering; no XML-specific equivalent."),

        // --- дополнительные бизнес-логика ---
        lib!("phonenumbers", "phonenumber", "Phone number parsing/validation. phonenumber wraps libphonenumber."),
        lib!("pycountry", "iso_country или manual lookup", "ISO country/language databases. iso_country provides country codes; for full databases embed your own data."),
        lib!("babel", "icu или fluent", "Internationalization utilities. icu wraps ICU4C; fluent handles message localization."),
        lib!("pyuca", "unicode-normalization", "Unicode Collation Algorithm. unicode-normalization handles normalization; full collation needs ICU."),

        // --- дополнительные специальные крейты ---
        lib!("ftplib", "suppaftp", "FTP client. suppaftp is the modern maintained Rust FTP client."),
        lib!("googletrans", "reqwest + manual API calls", "Google Translate API (unofficial). Use reqwest to call Google Cloud Translation API directly."),
        lib!("textblob", "rust-bert", "Simple NLP tasks. rust-bert handles modern transformer-based NLP."),
        lib!("gensim", "n/a — implement word2vec via candle or burn", "Topic modeling and word embeddings. No direct equivalent; use candle or burn for embedding models."),
        lib!("transformers", "rust-bert", "Hugging Face transformers. rust-bert loads and runs transformer models."),
        lib!("sentence-transformers", "rust-bert", "Sentence embeddings. rust-bert supports sentence transformer models."),
        lib!("langchain", "n/a — build LLM chains manually", "LLM application framework. No Rust equivalent; build chains with reqwest for API calls."),
        lib!("spacy", "rust-bert или manual tokenization", "Industrial NLP library. rust-bert covers transformer models; for tokenization use unicode-segmentation."),
        lib!("nltk", "rust-nlp или manual", "Natural language toolkit. No comprehensive equivalent; implement specific algorithms manually or use rust-bert."),
        lib!("fasttext", "fasttext (rust bindings)", "Text classification and embeddings. The fasttext crate wraps Facebook's FastText library."),
        lib!("bert-serving", "rust-bert", "BERT embeddings server. rust-bert loads BERT models for embeddings."),

        // --- дополнительные утилиты разработки ---
        lib!("ipdb", "n/a — используйте rust-gdb или rust-lldb", "Interactive debugger. Use rust-gdb, rust-lldb or IDE debuggers; no REPL-style debugger."),
        lib!("pdb", "n/a — используйте rust-gdb или rust-lldb", "Python debugger. Same as ipdb, use external debuggers."),
        lib!("icecream", "dbg! macro", "Debug print with context. The dbg! macro prints expressions with file/line info."),
        lib!("snoop", "tracing", "Advanced debugging with variable tracking. tracing with spans captures similar context."),
        lib!("memory_profiler", "valgrind или heaptrack", "Memory profiling. Use valgrind, heaptrack, or cargo-instruments on macOS."),
        lib!("line_profiler", "cargo-flamegraph", "Line-by-line performance profiling. cargo-flamegraph generates flame graphs; perf on Linux gives line-level data."),
        lib!("scalene", "cargo-flamegraph", "CPU/GPU/memory profiler. Use platform profilers; cargo-flamegraph for CPU."),
        lib!("py-spy", "cargo-flamegraph", "Sampling profiler. cargo-flamegraph or perf for similar sampling."),
        lib!("black", "rustfmt", "Code formatter. rustfmt is the standard Rust formatter."),
        lib!("pylint", "clippy", "Linter. clippy is Rust's official linter with hundreds of checks."),
        lib!("mypy", "built-in type checking", "Type checker. Rust's compiler performs type checking at compile time."),
        lib!("autopep8", "rustfmt", "Auto-format to PEP 8. rustfmt auto-formats to Rust style guide."),
        lib!("isort", "rustfmt", "Import sorting. rustfmt handles import organization."),
        lib!("flake8", "clippy", "Style guide enforcement. clippy checks style and correctness."),
        lib!("bandit", "cargo-audit", "Security linter. cargo-audit checks dependencies for known vulnerabilities."),
        lib!("safety", "cargo-audit", "Dependency vulnerability scanner. cargo-audit does the same for Rust."),
        lib!("pre-commit", "git hooks + cargo fmt/clippy", "Git hook framework. Use git hooks to run cargo fmt and clippy before commit."),

        // --- дополнительные игры/графика ---
        lib!("pyglet", "winit + wgpu", "OpenGL window and multimedia. winit creates windows; wgpu provides GPU rendering."),
        lib!("pyopengl", "gl или glow", "OpenGL bindings. The gl crate (or glow for safe wrappers) provides OpenGL access."),
        lib!("moderngl", "wgpu", "Modern OpenGL wrapper. wgpu is the modern GPU API, cross-platform and safe."),
        lib!("panda3d", "bevy", "3D game engine. bevy is a modern ECS-based game engine with 2D/3D support."),
        lib!("arcade", "macroquad", "Easy 2D game creation. macroquad provides simple 2D rendering and input."),
        lib!("cocos2d", "ggez", "2D game framework. ggez or macroquad fill the same role."),
        lib!("pygame", "macroquad", "2D game creation. macroquad provides simple rendering, audio, and input."),
        lib!("ursina", "bevy", "3D game engine with Python-like API. bevy is Rust's modern ECS game engine."),

        // --- дополнительные научные вычисления ---
        lib!("sympy", "n/a — символьная математика ограничена", "Symbolic mathematics. No full equivalent; some symbolic work possible via manual expression trees."),
        lib!("mpmath", "rug", "Arbitrary-precision floating point. rug wraps GMP/MPFR for high-precision math."),
        lib!("gmpy2", "rug", "GMP/MPFR bindings. rug is the same for Rust."),
        lib!("statsmodels", "linfa или statrs", "Statistical modeling. linfa covers ML models; statrs covers distributions and tests."),
        lib!("lifelines", "n/a — survival analysis hand-coded", "Survival analysis. No direct crate; implement using statrs or ndarray."),
        lib!("cvxpy", "n/a — optimization via argmin", "Convex optimization. No direct DSL; use argmin for optimization algorithms."),
        lib!("pulp", "good_lp", "Linear programming. good_lp provides an LP modeling interface."),
        lib!("pyomo", "n/a — manual optimization", "Optimization modeling. No equivalent; build models with argmin or external solvers."),
        lib!("numba", "built-in performance", "JIT compilation. Rust is compiled ahead-of-time, so no JIT needed."),
        lib!("cython", "built-in performance", "Python performance via C. Rust is already fast; no wrapper needed."),

        // --- дополнительные облачные SDK ---
        lib!("google-cloud-storage", "cloud-storage", "Google Cloud Storage client. cloud-storage provides an async client."),
        lib!("azure-storage-blob", "azure_storage_blobs", "Azure Blob Storage. The azure_storage_blobs crate wraps the REST API."),
        lib!("oci", "n/a — use HTTP client for OCI API", "Oracle Cloud Infrastructure SDK. Use reqwest to call OCI REST APIs directly."),
        lib!("google-cloud-pubsub", "cloud-pubsub", "Google Pub/Sub messaging. cloud-pubsub provides async pub/sub client."),
        lib!("azure-servicebus", "azure_messaging_servicebus", "Azure Service Bus messaging. azure_messaging_servicebus handles queues and topics."),
        lib!("aws-lambda", "aws-sdk-lambda", "AWS Lambda client. aws-sdk-lambda invokes and manages Lambda functions."),
        lib!("google-cloud-functions", "reqwest + manual API", "Google Cloud Functions client. Use reqwest to call Cloud Functions APIs."),
        lib!("azure-functions", "reqwest + manual API", "Azure Functions client. Use reqwest for HTTP-triggered functions."),
        lib!("digitalocean", "digitalocean", "DigitalOcean API client. The digitalocean crate wraps the REST API."),
        lib!("linode", "reqwest + manual API", "Linode API client. Use reqwest to call Linode APIs directly."),
        lib!("hetzner", "hcloud", "Hetzner Cloud API. hcloud provides a typed client for Hetzner resources."),
        lib!("cloudflare", "cloudflare", "Cloudflare API client. The cloudflare crate handles DNS, Workers, and KV."),
        lib!("terraform", "n/a — write HCL or use CDK", "Infrastructure as code. Write Terraform HCL directly or use AWS CDK in Rust."),
        lib!("pulumi", "pulumi_rust", "Infrastructure as code in native Rust. pulumi_rust builds cloud resources programmatically."),

        // --- дополнительные блокчейн/крипто ---
        lib!("web3", "web3", "Ethereum and Web3 interactions. The web3 crate talks to Ethereum nodes."),
        lib!("eth-account", "ethers", "Ethereum account utilities. ethers provides wallet and signing."),
        lib!("bitcoin", "bitcoin", "Bitcoin protocol types and operations. The bitcoin crate handles addresses, transactions, scripts."),
        lib!("solana", "solana-sdk", "Solana blockchain SDK. solana-sdk handles transactions, accounts, and RPC calls."),
        lib!("substrate", "substrate-api-client", "Substrate/Polkadot interaction. substrate-api-client connects to Substrate chains."),
        lib!("near", "near-sdk", "NEAR protocol SDK. near-sdk builds smart contracts."),
        lib!("tezos", "tezos-api", "Tezos blockchain client. tezos-api wraps RPC calls."),
        lib!("cardano", "cardano-serialization-lib", "Cardano transaction building. cardano-serialization-lib constructs transactions."),
        lib!("cosmos", "cosmrs", "Cosmos SDK client. cosmrs interacts with Cosmos chains."),
        lib!("algorand", "algonaut", "Algorand SDK. algonaut handles transactions and queries."),

        // --- дополнительные конфигурации ---
        lib!("hydra", "config", "Hierarchical configuration. The config crate reads layered config from files and env vars."),
        lib!("dynaconf", "config", "Dynamic multi-environment settings. config supports multiple sources and environments."),
        lib!("confuse", "config", "Configuration library. config is the standard choice."),
        lib!("python-decouple", "dotenv + envy", "Settings from environment. Combine dotenv to load .env with envy to deserialize."),
        lib!("configobj", "ini", "INI-style config parser. The ini crate parses INI files."),
        lib!("configargparse", "clap + config", "Combined CLI and config file parsing. Use clap for CLI with config for files."),

        // --- дополнительные workflow/task ---
        lib!("luigi", "n/a — build task DAGs manually", "Workflow management. No direct equivalent; use async tasks or manual dependency graphs."),
        lib!("airflow", "n/a — build pipelines with tokio", "Data pipeline orchestration. No Rust equivalent; build pipelines with tokio for orchestration."),
        lib!("prefect", "n/a — build pipelines with tokio", "Dataflow automation platform. No Rust equivalent; build with tokio for task orchestration."),
        lib!("dagster", "n/a — build pipelines manually", "Data orchestration platform. No direct equivalent; build pipelines with Rust async primitives."),
        lib!("dbt", "n/a — SQL transformation tool", "Data transformation tool. No Rust equivalent; run dbt directly or build SQL transformations manually."),
        lib!("great_expectations", "n/a — manual validation", "Data validation framework. No direct equivalent; build validation logic manually with assertions."),

        // --- дополнительные мониторинг/метрики ---
        lib!("prometheus_client", "prometheus", "Prometheus metrics client. The prometheus crate exposes metrics for scraping."),
        lib!("statsd", "cadence", "StatsD client for metrics. cadence sends StatsD metrics over UDP."),
        lib!("datadog", "ddmetric или reqwest", "Datadog APM and metrics. Use ddmetric for metrics or reqwest to call the Datadog API directly."),
        lib!("sentry-sdk", "sentry", "Error tracking and monitoring. The sentry crate captures errors and sends them to Sentry."),
        lib!("newrelic", "reqwest + manual API", "New Relic APM. Use reqwest to call New Relic APIs directly."),
        lib!("elastic-apm", "reqwest + manual API", "Elastic APM client. Use reqwest to send APM data to Elastic."),
        lib!("opencensus", "opentelemetry", "Distributed tracing. opentelemetry is the successor standard."),
        lib!("opentelemetry", "opentelemetry", "Observability framework. The opentelemetry crate provides tracing, metrics, and logs."),
        lib!("jaeger-client", "opentelemetry-jaeger", "Jaeger distributed tracing. opentelemetry-jaeger exports traces to Jaeger."),
        lib!("py-zipkin", "opentelemetry-zipkin", "Zipkin distributed tracing. opentelemetry-zipkin exports traces to Zipkin."),
        lib!("prefect", "n/a — use tokio for workflows", "Modern workflow orchestration. Build with tokio; no dedicated library."),

        // --- дополнительные документация ---
        lib!("sphinx", "mdbook или rustdoc", "Documentation generator. rustdoc generates API docs from doc comments; mdbook builds books from Markdown."),
        lib!("mkdocs", "mdbook", "Markdown documentation site generator. mdbook does the same."),
        lib!("pdoc", "rustdoc", "Auto-generate API docs. rustdoc is built into cargo doc."),

        // --- Machine Learning (Extended) ---
        lib!("keras", "burn или tch", "High-level neural network API. burn is pure Rust; tch wraps PyTorch's libtorch."),
        lib!("mxnet", "tch", "Deep learning framework. No direct bindings; use tch for similar tensor operations."),
        lib!("jax", "candle", "Composable transformations of array programs. candle provides similar tensor operations with automatic differentiation."),
        lib!("flax", "burn", "Neural network library for JAX. burn provides similar high-level NN APIs in pure Rust."),
        lib!("optuna", "n/a — hyperparameter tuning hand-coded", "Hyperparameter optimization framework. No direct equivalent; implement grid/random search manually."),
        lib!("ray", "n/a — distributed computing manual", "Distributed computing framework. Use tokio for async distribution or build custom distributed systems."),
        lib!("horovod", "n/a — distributed training manual", "Distributed deep learning. No direct equivalent; use multiple processes with MPI bindings if needed."),
        lib!("mlflow", "n/a — experiment tracking manual", "ML experiment tracking. No Rust equivalent; use MLflow server directly via HTTP API."),
        lib!("wandb", "reqwest + manual API", "Weights & Biases experiment tracking. Use reqwest to call W&B API directly."),
        lib!("sacred", "n/a — experiment tracking manual", "Experiment configuration and tracking. No equivalent; implement logging manually."),
        lib!("dvc", "n/a — version control tool", "Data version control. Run DVC CLI directly; no Rust API."),
        lib!("tensorboard", "n/a — use TensorBoard server", "TensorFlow visualization toolkit. Write logs and use TensorBoard server to visualize."),
        lib!("fastai", "burn", "High-level deep learning library. burn provides similar high-level abstractions."),
        lib!("catboost", "n/a — bindings limited", "Gradient boosting library. No maintained Rust bindings; call via FFI or use XGBoost instead."),
        lib!("prophet", "n/a — time series forecasting manual", "Time series forecasting. No direct equivalent; implement ARIMA or similar manually with statrs."),
        lib!("auto-sklearn", "n/a — AutoML manual", "Automated machine learning. No Rust equivalent; build pipelines manually."),
        lib!("tpot", "n/a — AutoML manual", "Genetic programming for AutoML. No equivalent; implement evolutionary algorithms manually."),
        lib!("huggingface_hub", "reqwest + manual API", "Hub for downloading models. Use reqwest to call Hugging Face API and download files."),
        lib!("datasets", "reqwest + manual loading", "Hugging Face datasets library. Download datasets via reqwest and parse manually."),
        lib!("tokenizers", "tokenizers", "Fast tokenizers from Hugging Face. The tokenizers crate is the official Rust implementation."),
        lib!("sentencepiece", "sentencepiece", "Unsupervised text tokenizer. The sentencepiece crate wraps the C++ library."),

        // --- Data Science (Extended) ---
        lib!("vaex", "polars", "Out-of-core DataFrames for large datasets. polars handles large datasets efficiently with lazy evaluation."),
        lib!("modin", "polars", "Parallel pandas. polars is fast and parallel by default."),
        lib!("cudf", "polars", "GPU DataFrames. polars runs on CPU but is highly optimized; for GPU use CUDA bindings manually."),
        lib!("rapids", "n/a — GPU acceleration manual", "GPU-accelerated data science. No direct equivalent; use CUDA bindings for GPU work."),
        lib!("awkward", "n/a — nested data structures manual", "Nested, variable-sized data. No direct equivalent; model with nested Vec or custom types."),
        lib!("uproot", "n/a — ROOT file reading limited", "Read ROOT files from particle physics. No maintained Rust equivalent; use C++ bindings or oxyroot (experimental)."),
        lib!("zarr", "zarr", "Chunked, compressed N-dimensional arrays. The zarr crate reads and writes Zarr format."),
        lib!("xarray", "ndarray + manual indexing", "N-dimensional labeled arrays. ndarray provides arrays; add labels manually with HashMap."),
        lib!("datatable", "polars", "Fast multi-threaded data manipulation. polars provides similar performance."),
        lib!("pyjanitor", "polars method chaining", "Clean data with method chaining. polars supports method chaining for transformations."),
        lib!("pandera", "n/a — DataFrame validation manual", "DataFrame schema validation. No equivalent; validate with assertions or custom logic."),
        lib!("xlwings", "calamine + rust_xlsxwriter", "Excel automation. calamine reads; rust_xlsxwriter writes. No VBA/macro support."),

        // --- Web Scraping (Extended) ---
        lib!("playwright", "fantoccini или thirtyfour", "Browser automation. fantoccini talks to WebDriver; thirtyfour provides async WebDriver client."),
        lib!("httpx", "reqwest", "Modern async HTTP client with HTTP/2. reqwest supports HTTP/2 and connection pooling."),
        lib!("requests-cache", "reqwest + cached", "HTTP caching. Combine reqwest with the cached crate for memoization."),
        lib!("mechanize", "reqwest + scraper", "Stateful web browsing. Use reqwest with cookie jars plus scraper for form parsing."),
        lib!("parsel", "scraper", "CSS selector extraction. scraper provides the same CSS selector API."),
        lib!("extruct", "scraper + manual parsing", "Extract structured data from HTML. Use scraper with custom logic for microdata/JSON-LD."),
        lib!("htmlmin", "minify-html", "HTML minification. minify-html minifies HTML for smaller payloads."),
        lib!("cssselect", "scraper", "CSS selector implementation. scraper uses the same selectors internally."),
        lib!("pyquery", "scraper", "jQuery-like HTML manipulation. scraper provides similar selection and traversal."),
        lib!("MechanicalSoup", "reqwest + scraper", "Browser simulation for scraping. Combine reqwest for requests with scraper for parsing."),

        // --- Testing (Extended) ---
        lib!("tox", "cargo with workspaces", "Test automation across environments. Use cargo workspaces and CI for multi-environment testing."),
        lib!("nox", "cargo-make или just", "Python test automation. cargo-make or just run tasks across environments."),
        lib!("pytest-cov", "cargo-tarpaulin", "Test coverage for pytest. cargo-tarpaulin generates coverage reports."),
        lib!("factory-boy", "n/a — test fixtures manual", "Test fixtures factory. No direct equivalent; build fixtures with builder pattern or rstest."),
        lib!("pytest-mock", "mockall", "Mocking for pytest. mockall generates mocks via macros."),
        lib!("pytest-django", "#[test] + database setup", "Django-specific testing. Use #[test] with test database setup manually."),
        lib!("pytest-bdd", "cucumber_rust", "Behavior-driven development. cucumber_rust provides Gherkin feature support."),
        lib!("behave", "cucumber_rust", "BDD framework. cucumber_rust implements Cucumber-style testing."),
        lib!("locust", "goose", "Load testing framework. goose provides similar load testing with async workers."),
        lib!("tavern", "n/a — API testing manual", "API testing framework. Write tests with reqwest in standard #[test] functions."),
        lib!("mimesis", "fake", "Fake data generator. fake generates similar test data."),
        lib!("vcrpy", "n/a — HTTP recording manual", "Record HTTP interactions. No equivalent; mock with mockito instead."),
        lib!("freezegun", "n/a — time mocking manual", "Mock datetime. No built-in support; use dependency injection for time sources."),
        lib!("pyfakefs", "n/a — filesystem mocking manual", "Mock filesystem. No equivalent; use tempfile for real temporary files."),

        // --- CLI (Extended) ---
        lib!("fire", "clap", "Generate CLI from function signatures. clap with derive macros provides similar automatic CLI generation."),
        lib!("docopt", "docopt", "CLI from usage patterns. The docopt crate implements the same usage-string parsing."),
        lib!("cement", "clap", "Advanced CLI framework. clap provides subcommands, plugins via features, and extensibility."),
        lib!("python-prompt-toolkit", "rustyline", "Interactive line editing and prompts. rustyline provides readline-like editing and history."),
        lib!("cmd", "rustyline", "Interactive command-line interpreter. rustyline with custom command parsing."),
        lib!("argcomplete", "clap_complete", "Shell completion for argparse. clap_complete generates completion scripts."),
        lib!("sh", "std::process::Command", "Shell command execution. std::process::Command runs external commands."),
        lib!("plumbum", "std::process::Command", "Shell combinators. std::process::Command with pipe chaining."),
        lib!("invoke", "cargo-make или just", "Task execution framework. cargo-make or just define tasks in TOML/justfile."),

        // --- Async (Extended) ---
        lib!("aiofiles", "tokio::fs", "Async file I/O. tokio::fs provides async file operations."),
        lib!("aioredis", "redis (async feature)", "Async Redis client. The redis crate with async feature supports tokio."),
        lib!("aiokafka", "rdkafka (async)", "Async Kafka client. rdkafka provides async producers and consumers."),
        lib!("asyncpg", "sqlx (postgres)", "Fast async PostgreSQL. sqlx with postgres feature provides async queries."),
        lib!("aiobotocore", "aws-sdk-rust", "Async AWS SDK. Official aws-sdk crates are async by default."),
        lib!("aiosmtpd", "lettre (async)", "Async SMTP server. lettre provides async transports; for servers build with tokio."),
        lib!("aio-pika", "lapin", "Async RabbitMQ. lapin is async-first for AMQP."),
        lib!("aiomysql", "sqlx (mysql)", "Async MySQL. sqlx with mysql feature provides async operations."),
        lib!("aiosqlite", "sqlx (sqlite)", "Async SQLite. sqlx with sqlite feature supports async queries."),
        lib!("httpcore", "hyper", "Low-level async HTTP. hyper is the underlying HTTP library for reqwest and axum."),

        // --- Messaging (Extended) ---
        lib!("kombu", "lapin", "Messaging library for AMQP. lapin provides AMQP 0.9.1 client for RabbitMQ."),
        lib!("nats-py", "nats", "NATS messaging client. The nats crate is the official Rust client."),
        lib!("pulsar-client", "pulsar", "Apache Pulsar client. The pulsar crate provides async producer/consumer."),
        lib!("azure-eventhub", "azure_messaging_eventhubs", "Azure Event Hubs. azure_messaging_eventhubs provides async event streaming."),
        lib!("google-cloud-pubsub", "cloud-pubsub", "Google Pub/Sub. cloud-pubsub provides async messaging."),
        lib!("sqs", "aws-sdk-sqs", "AWS SQS client. aws-sdk-sqs provides queue operations."),
        lib!("sns", "aws-sdk-sns", "AWS SNS client. aws-sdk-sns handles pub/sub notifications."),
        lib!("rabbitmq", "lapin", "RabbitMQ client. lapin is the async AMQP client."),
        lib!("celery", "n/a — distributed tasks manual", "Distributed task queue. Build with lapin + custom worker or use a job queue pattern."),
        lib!("rq", "n/a — Redis queue manual", "Redis-based task queue. Build with redis crate and custom worker loop."),
        lib!("dramatiq", "n/a — task queue manual", "Task processing framework. Build with message queue + workers manually."),
        lib!("huey", "n/a — task queue manual", "Lightweight task queue. Implement with redis or database as queue backend."),

        // --- Caching (Extended) ---
        lib!("diskcache", "cacache", "Disk-based cache. cacache provides content-addressable disk caching."),
        lib!("cachetools", "lru или cached", "In-memory caching utilities. lru provides LRU cache; cached adds memoization macros."),
        lib!("dogpile.cache", "cached", "Caching API with locking. The cached crate provides similar memoization."),
        lib!("beaker", "n/a — session caching manual", "Session and caching library. Build session caching with redis or database manually."),

        // --- Serialization (Extended) ---
        lib!("thrift", "thrift", "Apache Thrift serialization. The thrift crate implements the protocol and code generation."),
        lib!("flatbuffers", "flatbuffers", "Memory-efficient serialization. flatbuffers provides zero-copy deserialization."),
        lib!("capnproto", "capnp", "Cap'n Proto serialization. capnp is the Rust implementation with schema compiler."),
        lib!("ubjson", "n/a — limited support", "Universal Binary JSON. No maintained Rust implementation; use MessagePack or CBOR instead."),
        lib!("ion-python", "ion-rust", "Amazon Ion data format. ion-rust reads and writes Ion text and binary formats."),
        lib!("bencodepy", "bencode", "Bencode serialization for BitTorrent. The bencode crate handles the format."),
        lib!("rlp", "rlp", "Recursive Length Prefix for Ethereum. The rlp crate implements Ethereum's encoding."),

        // --- Networking (Extended) ---
        lib!("dpkt", "n/a — packet crafting manual", "Packet creation and parsing. Build packets manually with byte buffers or use etherparse."),
        lib!("impacket", "n/a — network protocols manual", "Network protocol implementations. No equivalent; implement protocols with tokio primitives."),
        lib!("pyshark", "pcap", "Packet capture via tshark. The pcap crate provides packet capture; parse with custom logic."),
        lib!("ldap3", "ldap3", "LDAP client. The ldap3 crate provides async LDAP operations."),
        lib!("netmiko", "russh", "Multi-vendor SSH library. russh provides SSH client functionality."),
        lib!("napalm", "n/a — network automation manual", "Network automation library. Build with russh and custom command logic."),
        lib!("nornir", "n/a — network automation manual", "Network automation framework. No equivalent; build with async Rust and device libraries."),
        lib!("telnetlib", "telnet", "Telnet client. The telnet crate implements the protocol."),
        lib!("dns.resolver", "trust-dns-resolver", "DNS resolution. trust-dns-resolver provides async DNS lookups."),
        lib!("publicsuffix", "publicsuffix", "Public Suffix List parsing. The publicsuffix crate extracts domains from URLs."),

        // --- Graphics (Extended) ---
        lib!("glfw", "glfw", "OpenGL window management. The glfw crate wraps GLFW for window creation."),
        lib!("sdl2", "sdl2", "SDL2 multimedia library. The sdl2 crate wraps SDL2 for rendering, audio, input."),
        lib!("vulkan", "vulkano или ash", "Vulkan graphics API. vulkano provides safe wrappers; ash is low-level bindings."),
        lib!("wgpu", "wgpu", "Modern GPU API. wgpu provides cross-platform GPU access (WebGPU standard)."),
        lib!("vispy", "wgpu + custom rendering", "Scientific visualization. No direct equivalent; build visualizations with wgpu."),
        lib!("mayavi", "n/a — 3D visualization manual", "3D scientific visualization. No Rust equivalent; use plotting libraries or external tools."),
        lib!("glumpy", "wgpu", "OpenGL visualization. Use wgpu for modern GPU-accelerated rendering."),
        lib!("vpython", "bevy или wgpu", "3D physics visualization. bevy provides 3D rendering; add physics separately."),

        // --- Audio/Video (Extended) ---
        lib!("moviepy", "ffmpeg bindings", "Video editing. Use ffmpeg-next or call ffmpeg CLI for video processing."),
        lib!("av", "ffmpeg-next", "Pythonic bindings to ffmpeg. ffmpeg-next wraps libav/ffmpeg libraries."),
        lib!("pyav", "ffmpeg-next", "FFmpeg bindings. ffmpeg-next provides the same access to libav."),
        lib!("sounddevice", "cpal", "Cross-platform audio I/O. cpal provides low-level audio stream access."),
        lib!("pyaudio", "cpal", "PortAudio bindings. cpal is pure Rust and cross-platform for audio I/O."),
        lib!("wave", "hound", "WAV file I/O. hound reads and writes WAV files."),
        lib!("pydub", "rodio + hound", "Audio manipulation. rodio for playback; hound for WAV; combine for processing."),
        lib!("audioread", "symphonia", "Audio file decoding. symphonia decodes many formats (mp3, flac, etc)."),
        lib!("aubio", "n/a — audio analysis manual", "Audio analysis. No direct equivalent; use rustfft for spectral analysis."),
        lib!("essentia", "n/a — audio analysis manual", "Audio analysis framework. No Rust equivalent; implement DSP manually."),
        lib!("madmom", "n/a — audio ML manual", "Audio/music information retrieval. No equivalent; build models with burn or tch."),
        lib!("librosa", "n/a — audio DSP manual", "Audio and music analysis. Use rustfft, rubato for resampling, or custom DSP."),
        lib!("soundfile", "hound или symphonia", "Read/write audio files. hound for WAV; symphonia for multi-format reading."),
        lib!("python-vlc", "libvlc", "VLC media player bindings. The vlc crate wraps libvlc."),
        lib!("mutagen", "audiotags", "Audio metadata editing. audiotags reads and writes ID3, MP4, FLAC tags."),

        // --- Geospatial (Extended) ---
        lib!("rasterio", "gdal", "Raster geospatial data. The gdal crate wraps GDAL for reading geospatial rasters."),
        lib!("fiona", "gdal", "Vector geospatial data. gdal also handles vector formats via OGR."),
        lib!("pyproj", "proj", "Cartographic projections. The proj crate wraps PROJ for coordinate transformations."),
        lib!("cartopy", "n/a — map plotting manual", "Geospatial data visualization. No equivalent; generate map tiles or use external tools."),
        lib!("osmnx", "osm-xml або osmpbf", "OpenStreetMap networks. osm-xml or osmpbf parse OSM data; build graphs manually."),
        lib!("geographiclib", "geographiclib-rs", "Geodesic calculations. geographiclib-rs implements geodesic algorithms."),
        lib!("pysal", "n/a — spatial analysis manual", "Spatial analysis library. No equivalent; implement spatial stats with geo crate."),
        lib!("geojson", "geojson", "GeoJSON (de)serialization. The geojson crate works with serde."),
        lib!("s2geometry", "s2", "S2 geometry library. The s2 crate implements Google's S2 spherical geometry."),
        lib!("h3", "h3o", "Hexagonal hierarchical geospatial indexing. h3o is a pure Rust H3 implementation."),
        lib!("pygeohash", "geohash", "Geohash encoding/decoding. The geohash crate implements the algorithm."),
        lib!("rtree", "rstar", "R-tree spatial indexing. rstar provides n-dimensional R* tree."),

        // --- Finance (Extended) ---
        lib!("quantlib", "n/a — quantitative finance manual", "Quantitative finance library. No direct equivalent; implement pricing models manually."),
        lib!("zipline", "n/a — backtesting manual", "Algorithmic trading backtester. No Rust equivalent; build backtesting with historical data."),
        lib!("backtrader", "n/a — backtesting manual", "Trading backtesting framework. Build custom backtesting with data feeds and strategy logic."),
        lib!("ta-lib", "ta", "Technical analysis indicators. The ta crate implements common technical indicators."),
        lib!("ccxt", "n/a — crypto exchange APIs manual", "Cryptocurrency exchange API. Use reqwest to call exchange APIs directly."),
        lib!("alpaca-trade-api", "reqwest + manual API", "Alpaca trading API. Use reqwest to call Alpaca REST API."),
        lib!("ib-insync", "n/a — Interactive Brokers manual", "Interactive Brokers API. Use TWS API via Rust FFI or build client with sockets."),
        lib!("pyfolio", "n/a — portfolio analysis manual", "Portfolio performance analysis. No equivalent; build analytics with ndarray and plotters."),
        lib!("empyrical", "n/a — financial metrics manual", "Financial statistics. Implement metrics like Sharpe ratio manually with statrs."),
        lib!("finviz", "reqwest + scraper", "Financial visualization scraper. Scrape finviz with reqwest + scraper."),

        // --- DevOps (Extended) ---
        lib!("fabric", "std::process::Command + russh", "Remote SSH command execution. Use russh for SSH + Command for local execution."),
        lib!("ansible", "n/a — use Ansible CLI", "Automation platform. Run ansible-playbook CLI; no Rust API needed."),
        lib!("salt", "n/a — use Salt CLI", "Configuration management. Call salt commands via std::process::Command."),
        lib!("docker-py", "bollard", "Docker API client. bollard wraps Docker Engine API."),
        lib!("kubernetes", "kube", "Kubernetes API client. The kube crate provides async client for k8s resources."),
        lib!("openshift", "kube", "OpenShift API. kube also works with OpenShift clusters."),
        lib!("vagrant", "n/a — use Vagrant CLI", "VM management. Call vagrant commands via std::process::Command."),
        lib!("paramiko", "russh", "SSH2 protocol library. russh is pure Rust SSH implementation."),
        lib!("gitpython", "git2", "Git repository manipulation. git2 wraps libgit2 for repository operations."),
        lib!("dulwich", "git2", "Pure-Python Git implementation. git2 provides the same Git operations."),
        lib!("pygithub", "octocrab", "GitHub API client. octocrab wraps GitHub REST API."),
        lib!("pybitbucket", "reqwest + manual API", "Bitbucket API client. Use reqwest to call Bitbucket API."),
        lib!("python-gitlab", "reqwest + manual API", "GitLab API client. Use reqwest to call GitLab REST API."),
        lib!("terraform-python", "n/a — use Terraform CLI", "Terraform wrapper. Call terraform commands via std::process::Command."),

        // --- Documentation (Extended) ---
        lib!("pydoc-markdown", "rustdoc", "API documentation generator. rustdoc generates docs from comments automatically."),
        lib!("portray", "mdbook", "Documentation from docstrings. Use rustdoc for API docs and mdbook for guides."),
        lib!("quartodoc", "rustdoc", "Quarto documentation. Use rustdoc for API documentation."),
        lib!("docutils", "pulldown-cmark", "Documentation utilities. pulldown-cmark parses Markdown; restructured text has no equivalent."),
        lib!("readme_renderer", "pulldown-cmark", "Render README for PyPI. pulldown-cmark renders Markdown to HTML."),

        // --- Code Quality (Extended) ---
        lib!("ruff", "clippy", "Fast Python linter. clippy is Rust's linter with hundreds of checks."),
        lib!("pyright", "built-in type checking", "Static type checker. Rust's compiler checks types at compile time."),
        lib!("pyre", "built-in type checking", "Type checker from Facebook. Rust has built-in type checking."),
        lib!("vulture", "n/a — dead code detection manual", "Dead code finder. cargo-udeps finds unused dependencies; cargo-machete finds unused code."),
        lib!("radon", "cargo-geiger", "Code complexity metrics. cargo-geiger measures unsafe code; write complexity tools manually."),
        lib!("mccabe", "n/a — complexity manual", "Cyclomatic complexity. No standard tool; analyze control flow manually."),
        lib!("pycodestyle", "rustfmt", "Style guide checker. rustfmt enforces Rust style automatically."),
        lib!("pydocstyle", "clippy (missing_docs lint)", "Docstring checker. Enable clippy's missing_docs lint to enforce documentation."),
        lib!("interrogate", "clippy (missing_docs)", "Documentation coverage. clippy's missing_docs warns on undocumented public items."),
        lib!("darglint", "n/a — docstring linting manual", "Docstring argument checker. No equivalent; keep doc comments accurate manually."),
        lib!("pyupgrade", "cargo fix", "Syntax upgrader. cargo fix applies automated fixes from compiler and clippy."),
        lib!("autoflake", "n/a — unused import removal manual", "Remove unused imports. Compiler warns about unused imports; remove manually."),
        lib!("add-trailing-comma", "rustfmt", "Code formatting. rustfmt handles trailing commas automatically."),
        lib!("pyflakes", "compiler warnings", "Detect coding errors. Rust compiler catches similar errors at compile time."),

        // --- Build Tools (Extended) ---
        lib!("poetry", "cargo", "Dependency management. cargo handles dependencies, building, and publishing."),
        lib!("flit", "cargo", "Simple packaging. cargo provides similar simple configuration via Cargo.toml."),
        lib!("hatch", "cargo", "Modern project manager. cargo is the standard tool for Rust projects."),
        lib!("setuptools", "cargo", "Package distribution. cargo build and cargo publish handle packaging."),
        lib!("wheel", "cargo", "Built package format. cargo produces binary crates automatically."),
        lib!("twine", "cargo publish", "PyPI upload tool. cargo publish uploads to crates.io."),
        lib!("build", "cargo build", "Build frontend. cargo build is the standard build command."),
        lib!("pip-tools", "cargo", "Pin dependencies. Cargo.lock automatically pins exact versions."),
        lib!("pipenv", "cargo", "Virtual environments and deps. cargo isolates dependencies per project natively."),
        lib!("conda", "cargo", "Package and environment manager. cargo manages Rust packages; no environment isolation needed."),
        lib!("pyenv", "rustup", "Python version manager. rustup manages Rust toolchains and versions."),
        lib!("virtualenvwrapper", "n/a — not needed", "Virtualenv management. Cargo handles isolation without separate environments."),
        lib!("pex", "cargo build --release", "Executable Python packages. cargo produces native executables."),
        lib!("shiv", "cargo build --release", "Zipapp creator. cargo build creates standalone binaries."),
        lib!("pyinstaller", "cargo build --release", "Package Python apps. cargo build produces native executables."),
        lib!("py2exe", "cargo build --release", "Windows executables. cargo build --release --target x86_64-pc-windows-msvc."),
        lib!("cx_Freeze", "cargo build --release", "Create executables. cargo build produces platform-native executables."),
        lib!("briefcase", "cargo build --release", "Cross-platform app packaging. cargo with target specifications for cross-platform builds."),

        // --- Protocols (Extended) ---
        lib!("json-rpc", "jsonrpc-core", "JSON-RPC protocol. jsonrpc-core implements server and client."),
        lib!("xmlrpc", "xmlrpc", "XML-RPC protocol. The xmlrpc crate implements client and server."),
        lib!("msgpack-rpc", "rmp-serde + custom RPC", "MessagePack RPC. Use rmp-serde for serialization and build RPC layer manually."),
        lib!("capnproto-rpc", "capnp (with RPC)", "Cap'n Proto RPC. capnp includes RPC support in the protocol."),
        lib!("thrift", "thrift", "Apache Thrift RPC. The thrift crate implements client and server."),
        lib!("soap", "n/a — SOAP manual", "SOAP protocol. No modern equivalent; build XML-based SOAP manually or use savon for simpler cases."),
        lib!("graphql-core", "juniper або async-graphql", "GraphQL server. juniper and async-graphql implement GraphQL schemas and resolvers."),
        lib!("ariadne", "async-graphql", "Schema-first GraphQL. async-graphql provides schema building."),
        lib!("strawberry", "async-graphql", "Type-based GraphQL. async-graphql uses types to define schemas."),

        // --- Embedded (Extended) ---
        lib!("micropython", "embedded Rust", "Python for microcontrollers. Use embedded Rust directly with no_std."),
        lib!("circuitpython", "embedded Rust", "Python for embedded devices. embedded Rust with HAL crates targets hardware directly."),
        lib!("pyserial", "serialport", "Serial port communication. The serialport crate provides cross-platform serial I/O."),
        lib!("spidev", "spidev", "SPI device interface. The spidev crate wraps Linux SPI."),
        lib!("smbus2", "i2cdev або linux-embedded-hal", "I2C/SMBus interface. i2cdev provides I2C on Linux; linux-embedded-hal for embedded."),
        lib!("rpi.gpio", "rppal", "Raspberry Pi GPIO. rppal provides GPIO, SPI, I2C, PWM for Raspberry Pi."),
        lib!("adafruit-blinka", "rppal або embedded-hal", "CircuitPython hardware API. Use rppal for Pi or embedded-hal for generic hardware."),
        lib!("pigpio", "rppal", "Raspberry Pi GPIO library. rppal covers GPIO, PWM, and hardware interfaces."),
        lib!("gpiozero", "rppal", "Simple GPIO library. rppal provides similar high-level GPIO abstractions."),

        // --- GUI (Extended) ---
        lib!("tkinter", "egui або iced", "Standard GUI toolkit. egui for immediate mode or iced for reactive UIs."),
        lib!("pyqt", "qt_widgets (rust-qt)", "Qt bindings. rust-qt provides Qt bindings; or use egui/iced as pure-Rust alternatives."),
        lib!("pyside", "qt_widgets", "Official Qt bindings. rust-qt covers Qt; consider egui or iced for pure Rust."),
        lib!("wxpython", "n/a — use native GUI crates", "wxWidgets bindings. No maintained Rust bindings; use egui, iced, or gtk-rs."),
        lib!("gtk", "gtk-rs", "GTK bindings. gtk-rs wraps GTK 3 and GTK 4."),
        lib!("qtpy", "qt_widgets", "Qt abstraction layer. rust-qt covers Qt APIs."),
        lib!("dear-imgui", "imgui-rs", "Immediate mode GUI. imgui-rs wraps Dear ImGui for debug UIs."),
        lib!("pyimgui", "imgui-rs", "Dear ImGui bindings. imgui-rs is the official Rust binding."),
        lib!("flexx", "n/a — web-based UI manual", "Web-based GUI toolkit. Build web UIs with yew or leptos (Wasm) or serve HTML from axum."),
        lib!("eel", "tauri", "HTML/JS GUI for Python. tauri builds desktop apps with web frontends and Rust backends."),
        lib!("pywebview", "tauri або wry", "Webview window. tauri for full apps; wry is the underlying webview library."),
        lib!("remi", "tauri", "Web UI framework. tauri provides similar desktop app capabilities."),

        // --- Scientific (Extended) ---
        lib!("biopython", "n/a — bioinformatics manual", "Bioinformatics toolkit. No comprehensive equivalent; implement algorithms manually or use bio crate for basics."),
        lib!("rdkit", "n/a — cheminformatics manual", "Cheminformatics library. No Rust equivalent; call RDKit via FFI if needed."),
        lib!("mdanalysis", "n/a — molecular dynamics manual", "Molecular dynamics analysis. No direct equivalent; parse trajectories manually."),
        lib!("ase", "n/a — atomic simulation manual", "Atomic Simulation Environment. No equivalent; implement physics calculations manually."),
        lib!("pymatgen", "n/a — materials science manual", "Materials science library. No Rust equivalent; implement crystal structure calculations manually."),
        lib!("cclib", "n/a — computational chemistry manual", "Computational chemistry parser. No equivalent; parse output files with custom parsers."),
        lib!("openbabel", "n/a — chemistry toolbox manual", "Chemical file format converter. No Rust equivalent; call Open Babel via FFI."),
        lib!("pyscf", "n/a — quantum chemistry manual", "Quantum chemistry package. No Rust bindings; call PySCF via FFI if needed."),
        lib!("qiskit", "n/a — quantum computing manual", "Quantum computing SDK. No Rust equivalent; use Qiskit's REST API via reqwest."),
        lib!("cirq", "n/a — quantum computing manual", "Quantum computing framework. No Rust equivalent; call Cirq via API."),
        lib!("pennylane", "n/a — quantum ML manual", "Quantum machine learning. No Rust equivalent; use external quantum APIs."),
        lib!("astropy", "n/a — astronomy manual", "Astronomy library. No comprehensive equivalent; implement specific calculations with ndarray."),
        lib!("sunpy", "n/a — solar physics manual", "Solar physics toolkit. No equivalent; process FITS files with fitsio crate."),
        lib!("healpy", "n/a — spherical data manual", "HEALPix projection for astronomy. No Rust equivalent; implement algorithms manually."),

        // --- Time Series (Extended) ---
        lib!("tsfresh", "n/a — feature extraction manual", "Time series feature extraction. No equivalent; compute features manually with ndarray."),
        lib!("stumpy", "n/a — matrix profile manual", "Matrix profile for time series. No Rust equivalent; implement algorithms manually."),
        lib!("tslearn", "n/a — time series ML manual", "Time series machine learning. No equivalent; use linfa for ML on time series data."),
        lib!("sktime", "n/a — time series toolkit manual", "Time series learning toolkit. No equivalent; build models with burn or linfa."),
        lib!("pmdarima", "n/a — ARIMA manual", "Auto-ARIMA for forecasting. No equivalent; implement ARIMA with statrs."),
        lib!("statsforecast", "n/a — forecasting manual", "Statistical forecasting. No equivalent; implement models manually."),
        lib!("neuralprophet", "burn или tch", "Neural network forecasting. Use burn or tch to build forecasting models."),
        lib!("kats", "n/a — time series toolkit manual", "Time series analysis toolkit. No equivalent; build pipelines manually."),
        lib!("orbit", "n/a — Bayesian forecasting manual", "Bayesian time series. No equivalent; implement Bayesian models manually."),

        // --- Databases Advanced (Extended) ---
        lib!("sqlmodel", "sqlx или diesel", "SQL databases with Python types. sqlx or diesel provide typed query builders."),
        lib!("tortoise-orm", "sea-orm", "Async ORM. sea-orm is async-first ORM with migrations and relations."),
        lib!("peewee", "diesel", "Lightweight ORM. diesel provides similar expressive ORM."),
        lib!("pony", "diesel", "ORM with generator expressions. diesel uses type-safe query builders."),
        lib!("dataset", "sqlx", "Database as a dictionary. sqlx provides flexible query execution."),
        lib!("records", "sqlx", "SQL for humans. sqlx with compile-time checked queries."),
        lib!("alembic", "diesel_migrations или sea-orm-migration", "Database migrations. diesel_migrations or sea-orm-migration manage schema changes."),
        lib!("yoyo-migrations", "refinery", "Database migrations. refinery manages migrations across databases."),
        lib!("aerospike", "aerospike-client-rust", "Aerospike NoSQL database. aerospike-client-rust wraps the C client."),
        lib!("rethinkdb", "reql", "RethinkDB client. reql provides query building for RethinkDB."),
        lib!("couchdb", "reqwest + manual API", "CouchDB client. Use reqwest to call CouchDB HTTP API."),
        lib!("arango", "arangors", "ArangoDB client. arangors provides async multi-model database access."),
        lib!("dgraph", "dgraph-tonic", "DGraph client. dgraph-tonic provides gRPC client for graph database."),
        lib!("orientdb", "n/a — use HTTP API", "OrientDB client. Use reqwest to call OrientDB REST API."),
        lib!("tinydb", "sled", "Lightweight document database. sled is an embedded key-value store; build document layer on top."),
        lib!("pickledb", "sled або redb", "Lightweight key-value store. sled or redb provide embedded databases."),
        lib!("shelve", "sled", "Python object persistence. sled with serde for serialization."),
        lib!("dbm", "sled", "Database manager interface. sled is modern embedded database."),

        // --- Monitoring (Extended) ---
        lib!("pyinstrument", "cargo-flamegraph", "Statistical profiler. cargo-flamegraph generates flame graphs for performance analysis."),
        lib!("viztracer", "cargo-flamegraph", "Trace profiler. cargo-flamegraph or tracing for instrumentation."),
        lib!("austin", "cargo-flamegraph", "Frame stack sampler. Use cargo-flamegraph or platform profilers."),
        lib!("memray", "valgrind or heaptrack", "Memory profiler. Use valgrind or heaptrack for memory profiling."),
        lib!("tracemalloc", "valgrind", "Memory allocation tracking. valgrind tracks memory allocations."),
        lib!("guppy", "cargo-bloat", "Memory profiling. cargo-bloat shows binary size; valgrind tracks runtime memory."),
        lib!("objgraph", "n/a — no GC in Rust", "Object reference graph. Rust has no GC; track references via ownership analysis."),
        lib!("pympler", "valgrind", "Memory profiling. valgrind measures memory usage."),
        lib!("heartbeat", "n/a — health check manual", "Service health monitoring. Build health check endpoints in web frameworks."),
        lib!("healthcheck", "n/a — health check manual", "Health check framework. Implement health endpoints in axum or actix-web."),

        // --- Security (Extended) ---
        lib!("pwntools", "n/a — exploit dev manual", "CTF and exploit development. No equivalent; build exploits with std::process and socket primitives."),
        lib!("scapy", "pnet", "Packet manipulation. pnet provides low-level packet crafting and sniffing."),
        lib!("impacket", "n/a — network protocols manual", "Network protocol implementations. Implement SMB, Kerberos, etc. manually with tokio."),
        lib!("volatility", "n/a — memory forensics manual", "Memory forensics framework. No Rust equivalent; use Volatility directly."),
        lib!("yara-python", "yara-rust", "Malware identification. yara-rust provides YARA pattern matching."),
        lib!("capstone", "capstone", "Disassembly framework. The capstone crate wraps the Capstone engine."),
        lib!("keystone", "keystone", "Assembler framework. The keystone crate wraps Keystone assembler."),
        lib!("unicorn", "unicorn-engine", "CPU emulator. unicorn-engine wraps the Unicorn emulator."),
        lib!("angr", "n/a — symbolic execution manual", "Binary analysis framework. No equivalent; implement symbolic execution manually."),
        lib!("r2pipe", "n/a — use radare2 API", "Radare2 integration. Call radare2 via pipes or HTTP API with reqwest."),
        lib!("miasm", "n/a — reverse engineering manual", "Reverse engineering framework. No equivalent; use disassemblers directly."),
        lib!("bandit", "cargo-audit", "Security linter. cargo-audit scans for vulnerable dependencies."),
        lib!("pysecdump", "n/a — security scanning manual", "Security information dumper. Build security scanners with system APIs."),

        // --- Additional Web Frameworks ---
        lib!("responder", "axum", "Async web framework. axum provides similar async request handling."),
        lib!("hug", "axum", "API framework. axum with serde for typed APIs."),
        lib!("falcon", "actix-web", "Fast web framework for APIs. actix-web is similarly performance-focused."),
        lib!("morepath", "axum", "Model-driven web framework. axum with routers and extractors."),
        lib!("web2py", "actix-web + templates", "Full-stack framework. actix-web plus tera or askama for templates."),
        lib!("turbogears", "actix-web + ORM", "Full-stack framework. actix-web with diesel or sea-orm."),
        lib!("dash", "axum + HTML templates", "Data app framework. Build dashboards with axum serving HTML and charts."),
        lib!("streamlit", "axum + WebSockets", "Data science app builder. Build interactive apps with axum and htmx or Wasm frontend."),
        lib!("gradio", "axum + frontend", "ML demo interfaces. Build UIs with axum backend and web frontend."),
        lib!("fasthtml", "axum + askama", "HTML-first web framework. axum with askama templates for HTML generation."),

        // --- Additional HTTP Libraries ---
        lib!("httplib", "reqwest", "HTTP client. reqwest provides comprehensive HTTP client."),
        lib!("asks", "reqwest", "Async HTTP library. reqwest is async by default with tokio."),
        lib!("treq", "reqwest", "Twisted HTTP client. reqwest provides async HTTP with tokio."),
        lib!("grab", "reqwest + scraper", "Web scraping framework. Combine reqwest for fetching with scraper for parsing."),

        // --- Additional Data Processing ---
        lib!("petl", "polars", "ETL (Extract, Transform, Load). polars provides similar data transformation pipelines."),
        lib!("pyarrow", "arrow", "Apache Arrow. The arrow crate is the official Rust implementation."),
        lib!("fastparquet", "parquet", "Parquet file format. The parquet crate reads and writes Parquet files."),
        lib!("pyexcel", "calamine", "Spreadsheet file manipulation. calamine reads Excel; rust_xlsxwriter writes."),
        lib!("xmlschema", "quick-xml + validation", "XML schema validation. quick-xml parses; add validation logic manually."),
        lib!("dicttoxml", "quick-xml + serde", "Convert dict to XML. Use serde with quick-xml for struct to XML."),
        lib!("openpyxl-templates", "rust_xlsxwriter", "Excel templates. rust_xlsxwriter creates Excel files programmatically."),
        lib!("jsonschema", "jsonschema", "JSON Schema validation. The jsonschema crate validates JSON against schemas."),
        lib!("marshmallow-jsonschema", "schemars", "Generate JSON Schema. schemars generates JSON Schema from Rust types."),
        lib!("dataclasses-json", "serde_json", "Dataclass JSON serialization. serde_json with derive macros handles this."),
        lib!("pyyaml-include", "serde_yaml", "YAML with includes. Load and merge YAML files manually with serde_yaml."),

        // --- Additional CLI/TUI ---
        lib!("cement", "clap", "CLI application framework. clap provides comprehensive CLI building."),
        lib!("cliff", "clap", "Command-line framework. clap with subcommands covers this."),
        lib!("cleo", "clap", "Beautiful command-line interfaces. clap with colored for output styling."),
        lib!("python-nubia", "clap + rustyline", "Interactive CLI framework. Combine clap with rustyline for interactive shells."),
        lib!("textual", "ratatui", "TUI framework. ratatui builds modern terminal UIs with widgets."),
        lib!("urwid", "cursive", "Console UI library. cursive provides similar TUI building blocks."),
        lib!("npyscreen", "cursive", "TUI framework. cursive is the modern Rust equivalent."),
        lib!("py_cui", "ratatui", "Python CUI library. ratatui provides comprehensive TUI capabilities."),
        lib!("bullet", "dialoguer", "Interactive prompts. dialoguer provides menus and selections."),
        lib!("inquirer", "dialoguer", "Interactive CLI prompts. dialoguer covers questions, confirmations, selections."),
        lib!("pick", "dialoguer", "Picker for lists. dialoguer's Select and MultiSelect handle this."),
        lib!("beautysh", "n/a — shell formatting", "Shell script formatter. No Rust equivalent; use shfmt or format manually."),

        // --- Additional Validation ---
        lib!("jsonschema", "jsonschema", "JSON Schema validator. The jsonschema crate validates against schemas."),
        lib!("pydantic", "serde + validator", "Data validation via types. serde for structure; validator for rules."),
        lib!("colander", "validator", "Serialization and validation. validator provides field-level validation."),
        lib!("formencode", "validator", "Form validation. validator with serde for form handling."),
        lib!("wtforms", "n/a — form validation manual", "Form rendering and validation. Build forms manually; validate with validator."),
        lib!("django-rest-framework", "axum + serde", "REST framework. axum with serde for API serialization and validation."),

        // --- Additional Utilities ---
        lib!("more-itertools", "itertools", "Extended iterator tools. The itertools crate provides many additional combinators."),
        lib!("toolz", "itertools", "Functional utilities. itertools plus standard iterator methods cover this."),
        lib!("funcy", "itertools + std", "Functional tools. Combine itertools with closures and standard library."),
        lib!("boltons", "std + custom utilities", "Utility library. Many built into std; implement specific utilities as needed."),
        lib!("cytoolz", "itertools", "Cython-optimized toolz. itertools is already fast in Rust."),
        lib!("fn.py", "std functional features", "Functional programming. Rust has built-in closures, iterators, Option, Result."),
        lib!("returns", "Result and Option types", "Functional error handling. Rust's Result and Option provide similar monadic patterns."),
        lib!("pampy", "match expression", "Pattern matching. Rust's match expression is built-in and exhaustive."),
        lib!("deprecated", "deprecated attribute", "Deprecation warnings. #[deprecated] attribute marks deprecated items."),
        lib!("attrs", "struct + derive", "Class boilerplate. #[derive] macros generate methods automatically."),
        lib!("python-box", "serde_json::Value", "Dict access with dots. serde_json::Value provides similar dynamic access."),
        lib!("munch", "serde_json::Value", "Dot-accessible dict. serde_json::Value with bracket/dot-style access."),
        lib!("addict", "serde_json::Value", "Dict subclass with attribute access. serde_json::Value provides dynamic access."),
        lib!("bunch", "serde_json::Value", "Dict with attribute access. serde_json::Value or define structs."),
        lib!("easydict", "struct", "Dict to object. Define typed structs with serde."),
        lib!("python-decouple", "dotenv + envy", "Separate config from code. dotenv loads .env; envy deserializes to struct."),
        lib!("envparse", "envy", "Environment variable parsing. envy deserializes env vars to structs."),

        // --- Additional System Tools ---
        lib!("supervisor", "systemd або manual process mgmt", "Process control system. Use systemd for services or manage with std::process."),
        lib!("daemon", "daemonize", "Unix daemon creation. The daemonize crate creates background daemons."),
        lib!("python-daemon", "daemonize", "Daemonize Python programs. daemonize handles Unix daemon setup."),
        lib!("schedule", "tokio + tokio-cron-scheduler", "Job scheduling. tokio-cron-scheduler runs periodic tasks."),
        lib!("apscheduler", "tokio-cron-scheduler", "Advanced Python scheduler. tokio-cron-scheduler provides cron-like scheduling."),
        lib!("timeloop", "tokio::time::interval", "Simple periodic tasks. tokio::time::interval runs tasks at intervals."),
        lib!("croniter", "cron", "Cron expression parser. The cron crate parses and evaluates cron schedules."),
        lib!("python-crontab", "cron", "Cron management. The cron crate handles cron expressions."),
        lib!("pty", "portable-pty", "Pseudo-terminal handling. portable-pty provides cross-platform PTY."),
        lib!("pexpect", "rexpect", "Automate interactive programs. rexpect interacts with spawned processes."),

        // --- Additional Image/Video ---
        lib!("wand", "imagemagick-rust", "ImageMagick bindings. imagemagick-rust wraps ImageMagick; or use image crate."),
        lib!("pgmagick", "imagemagick-rust", "GraphicsMagick bindings. imagemagick-rust covers similar image manipulation."),
        lib!("thumbor", "image + resizing", "Image service. Use image crate with thumbnail generation manually."),
        lib!("pilkit", "image", "Image utilities for PIL. The image crate provides similar operations."),
        lib!("imagesize", "image", "Get image dimensions. The image crate reads dimensions without full decode."),
        lib!("qrcode", "qrcode", "QR code generation. The qrcode crate generates QR codes."),
        lib!("python-barcode", "barcoders", "Barcode generation. barcoders generates various barcode formats."),
        lib!("svgwrite", "svg", "SVG creation. The svg crate generates SVG documents."),
        lib!("cairosvg", "resvg", "SVG to PNG/PDF converter. resvg renders SVG to raster images."),
        lib!("svglib", "resvg", "SVG parser and renderer. resvg parses and renders SVG."),

        // --- Additional Compression ---
        lib!("python-lz4", "lz4", "LZ4 compression. The lz4 crate wraps LZ4 compression."),
        lib!("python-snappy", "snap", "Snappy compression. snap implements Snappy compression."),
        lib!("cramjam", "various compression crates", "Compression bindings. Use specific crates: flate2, zstd, lz4, snap."),
        lib!("blosc", "n/a — use specific codecs", "Meta-compressor. Use flate2, lz4, or zstd directly."),

        // --- Additional Parsing ---
        lib!("configparser", "ini", "INI parser. The ini crate handles INI files."),
        lib!("python-dotenv", "dotenv", "Load .env files. The dotenv crate loads environment variables from files."),
        lib!("environs", "envy + dotenv", "Environment variable parsing. Combine dotenv with envy for typed env vars."),
        lib!("parse", "scanf или manual parsing", "Simple string parsing. Use scanf crate or manual string methods."),
        lib!("simpleeval", "n/a — eval is unsafe", "Simple expression evaluator. Implement expression parser with nom or pest; avoid eval."),
        lib!("asteval", "n/a — eval manual", "AST-based expression evaluator. Build expression evaluator with nom or pest."),
        lib!("ply", "pest или lalrpop", "Lex/yacc for Python. pest uses PEG grammars; lalrpop uses LR parsing."),
        lib!("lark", "pest", "Parsing library. pest generates parsers from PEG grammars."),
        lib!("textx", "pest", "Meta-language for DSLs. pest builds parsers from grammar files."),

        // --- Additional Email ---
        lib!("yagmail", "lettre", "Simple email sending. lettre sends email over SMTP."),
        lib!("mailer", "lettre", "Email library. lettre handles email composition and sending."),
        lib!("premailer", "n/a — CSS inlining manual", "Inline CSS for email. No equivalent; process HTML with scraper and manual style inlining."),
        lib!("flanker", "mail-parser", "Email address parsing. mail-parser parses email messages and addresses."),
        lib!("validate-email", "validator (email feature)", "Email validation. validator crate validates email addresses."),
        lib!("mailbox", "mailparse", "Mailbox format handling. mailparse parses email messages."),

        // --- Additional Async/Concurrent ---
        lib!("concurrent.futures", "rayon або tokio", "High-level async/parallel execution. rayon for CPU-bound; tokio for I/O-bound."),
        lib!("gevent", "tokio", "Coroutine-based networking. tokio provides async runtime."),
        lib!("eventlet", "tokio", "Concurrent networking. tokio handles async I/O."),
        lib!("trio", "tokio", "Async I/O library. tokio is the standard async runtime."),
        lib!("anyio", "tokio", "Async abstraction layer. tokio provides the async runtime."),
        lib!("uvloop", "tokio", "Fast event loop. tokio is the standard Rust async runtime."),

        // --- Additional Blockchain/Crypto ---
        lib!("ecdsa", "ecdsa", "ECDSA signatures. The ecdsa crate implements elliptic curve signatures."),
        lib!("ed25519", "ed25519-dalek", "Ed25519 signatures. ed25519-dalek provides fast EdDSA."),
        lib!("pycryptodome", "ring или rust-crypto", "Cryptographic primitives. ring provides modern crypto; rust-crypto has legacy algorithms."),
        lib!("coincurve", "secp256k1", "secp256k1 bindings. The secp256k1 crate wraps libsecp256k1."),
        lib!("bip32", "bip32", "BIP32 hierarchical deterministic wallets. The bip32 crate implements HD wallets."),
        lib!("mnemonic", "bip39", "BIP39 mnemonic codes. The bip39 crate handles mnemonic generation and recovery."),

        // --- Additional Templating ---
        lib!("bottle-templates", "tera", "Bottle templating. tera provides similar template syntax."),
        lib!("genshi", "tera", "XML-based templates. tera handles templating; XML generation with quick-xml."),
        lib!("pystache", "mustache", "Mustache templates. The mustache crate implements Mustache rendering."),
        lib!("chevron", "mustache", "Mustache rendering. mustache provides the same logic-less templates."),

        // --- Additional String/Text ---
        lib!("inflect", "inflector", "Pluralization and singularization. inflector handles English inflections."),
        lib!("humanize", "chrono-humanize", "Human-readable values. chrono-humanize formats durations; implement number formatting manually."),
        lib!("python-dateutil", "chrono", "Date parsing and utilities. chrono handles parsing and arithmetic."),
        lib!("ftfy", "n/a — text fixing manual", "Fix text encoding issues. Handle with String methods and encoding detection."),
        lib!("emoji", "emojis", "Emoji handling. The emojis crate provides emoji data and lookup."),
        lib!("textdistance", "strsim", "String distance algorithms. strsim implements Levenshtein, Hamming, etc."),
        lib!("fuzzywuzzy", "strsim + custom logic", "Fuzzy string matching. strsim provides distance metrics; combine for fuzzy matching."),
        lib!("python-Levenshtein", "strsim", "Fast Levenshtein distance. strsim provides efficient implementations."),
        lib!("metaphone", "n/a — phonetic algorithms manual", "Phonetic algorithms. No crate; implement Soundex/Metaphone manually."),
        lib!("jellyfish", "strsim", "String comparison library. strsim covers distance metrics."),
    ]
    .into_iter()
    .collect()
}

/// Модули стандартной библиотеки Python, для которых не нужен внешний крейт (не показываем в списке).
pub fn stdlib_no_suggestion() -> &'static [&'static str] {
    &["sys", "typing", "abc", "functools", "copy", "io", "math", "string", "warnings", "traceback", "inspect", "contextlib"]
}

#[cfg(test)]
mod tests;
