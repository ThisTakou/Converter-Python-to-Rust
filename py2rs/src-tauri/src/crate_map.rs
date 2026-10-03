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

        // --- дополнительные утилиты разработки ---
        lib!("ipdb", "n/a — используйте rust-gdb или rust-lldb", "Interactive debugger. Use rust-gdb, rust-lldb or IDE debuggers; no REPL-style debugger."),
        lib!("pdb", "n/a — используйте rust-gdb или rust-lldb", "Python debugger. Same as ipdb, use external debuggers."),
        lib!("icecream", "dbg! macro", "Debug print with context. The dbg! macro prints expressions with file/line info."),
        lib!("snoop", "tracing", "Advanced debugging with variable tracking. tracing with spans captures similar context."),
        lib!("memory_profiler", "valgrind или heaptrack", "Memory profiling. Use valgrind, heaptrack, or cargo-instruments on macOS."),
        lib!("line_profiler", "cargo-flamegraph", "Line-by-line performance profiling. cargo-flamegraph generates flame graphs; perf on Linux gives line-level data."),
        lib!("scalene", "cargo-flamegraph", "CPU/GPU/memory profiler. Use platform profilers; cargo-flamegraph for CPU."),
        lib!("py-spy", "cargo-flamegraph", "Sampling profiler. cargo-flamegraph or perf for similar sampling."),

        // --- дополнительные игры/графика ---
        lib!("pyglet", "winit + wgpu", "OpenGL window and multimedia. winit creates windows; wgpu provides GPU rendering."),
        lib!("pyopengl", "gl или glow", "OpenGL bindings. The gl crate (or glow for safe wrappers) provides OpenGL access."),
        lib!("moderngl", "wgpu", "Modern OpenGL wrapper. wgpu is the modern GPU API, cross-platform and safe."),
        lib!("panda3d", "bevy", "3D game engine. bevy is a modern ECS-based game engine with 2D/3D support."),
        lib!("arcade", "macroquad", "Easy 2D game creation. macroquad provides simple 2D rendering and input."),
        lib!("cocos2d", "ggez", "2D game framework. ggez or macroquad fill the same role."),

        // --- дополнительные научные вычисления ---
        lib!("sympy", "n/a — символьная математика ограничена", "Symbolic mathematics. No full equivalent; some symbolic work possible via manual expression trees."),
        lib!("mpmath", "rug", "Arbitrary-precision floating point. rug wraps GMP/MPFR for high-precision math."),
        lib!("gmpy2", "rug", "GMP/MPFR bindings. rug is the same for Rust."),
        lib!("statsmodels", "linfa или statrs", "Statistical modeling. linfa covers ML models; statrs covers distributions and tests."),
        lib!("lifelines", "n/a — survival analysis hand-coded", "Survival analysis. No direct crate; implement using statrs or ndarray."),

        // --- дополнительные облачные SDK ---
        lib!("google-cloud-storage", "cloud-storage", "Google Cloud Storage client. cloud-storage provides an async client."),
        lib!("azure-storage-blob", "azure_storage_blobs", "Azure Blob Storage. The azure_storage_blobs crate wraps the REST API."),
        lib!("oci", "n/a — use HTTP client for OCI API", "Oracle Cloud Infrastructure SDK. Use reqwest to call OCI REST APIs directly."),

        // --- дополнительные блокчейн/крипто ---
        lib!("web3", "web3", "Ethereum and Web3 interactions. The web3 crate talks to Ethereum nodes."),
        lib!("eth-account", "ethers", "Ethereum account utilities. ethers provides wallet and signing."),
        lib!("bitcoin", "bitcoin", "Bitcoin protocol types and operations. The bitcoin crate handles addresses, transactions, scripts."),

        // --- дополнительные конфигурации ---
        lib!("hydra", "config", "Hierarchical configuration. The config crate reads layered config from files and env vars."),
        lib!("dynaconf", "config", "Dynamic multi-environment settings. config supports multiple sources and environments."),
        lib!("confuse", "config", "Configuration library. config is the standard choice."),

        // --- дополнительные workflow/task ---
        lib!("luigi", "n/a — build task DAGs manually", "Workflow management. No direct equivalent; use async tasks or manual dependency graphs."),
        lib!("airflow", "n/a — build pipelines with tokio", "Data pipeline orchestration. No Rust equivalent; build pipelines with tokio for orchestration."),
        lib!("prefect", "n/a — use tokio for workflows", "Modern workflow orchestration. Build with tokio; no dedicated library."),

        // --- дополнительные документация ---
        lib!("sphinx", "mdbook или rustdoc", "Documentation generator. rustdoc generates API docs from doc comments; mdbook builds books from Markdown."),
        lib!("mkdocs", "mdbook", "Markdown documentation site generator. mdbook does the same."),
        lib!("pdoc", "rustdoc", "Auto-generate API docs. rustdoc is built into cargo doc."),
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
