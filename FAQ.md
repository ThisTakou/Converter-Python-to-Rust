# Frequently Asked Questions (FAQ)

## General Questions

### What is py2rs?

py2rs is an LLM-powered desktop application that helps migrate Python projects to Rust. It uses AI models (like GPT-4, Claude, or local models via Ollama) to automatically translate Python code to idiomatic Rust while preserving functionality.

### How does it work?

py2rs analyzes your Python project, detects external libraries, suggests Rust crate equivalents, and then uses an LLM to translate each file to Rust. It also compiles the generated code to check for errors and provides a detailed report.

### Is it free?

The application itself is free and open source. However, you'll need access to an LLM:
- **Free option**: Use local models via Ollama or llama.cpp (no API costs)
- **Paid option**: Use cloud APIs like OpenAI, Anthropic, or OpenRouter (costs vary by provider)

## Installation & Setup

### What are the system requirements?

- **Operating System**: Windows, macOS, or Linux
- **Disk Space**: ~100MB for the application
- **For local models**: Additional 4-8GB for model weights (e.g., Qwen 2.5 Coder)
- **Internet**: Required for cloud LLM APIs (optional for local models)

### How do I install py2rs?

1. Download the latest release for your platform from GitHub releases
2. Install the application:
   - **Windows**: Run the `.msi` installer
   - **macOS**: Open the `.dmg` and drag to Applications
   - **Linux**: Run the `.AppImage` or install via package manager

### How do I set up a local LLM?

1. Install [Ollama](https://ollama.ai/)
2. Pull a code model: `ollama pull qwen2.5-coder:7b`
3. In py2rs, select "Local / OpenAI-compatible" provider
4. Set base URL to `http://localhost:11434/v1`
5. Set model name to `qwen2.5-coder`

## Usage

### Which Python projects work best?

py2rs works best with:
- Small to medium projects (< 10,000 lines)
- Projects with clear structure and standard libraries
- Pure Python code (not heavily dependent on C extensions)

It may struggle with:
- Very large codebases (>50,000 lines)
- Heavy use of Python-specific features (metaclasses, dynamic imports)
- Projects tightly coupled to Python-only frameworks

### How accurate is the migration?

Accuracy depends on several factors:
- **Model quality**: Larger models (GPT-4, Claude Opus) produce better results
- **Code complexity**: Simple, well-structured code migrates better
- **Library availability**: Common libraries have better Rust equivalents

Typical success rates:
- **Simple scripts**: 80-95% compile without errors
- **Web services**: 60-80% (depends on framework)
- **Data science**: 50-70% (NumPy/Pandas equivalents differ)

### Can I customize the migration?

Yes! You can:
- Edit the Rust crate mappings for Python libraries
- Add custom documentation for modules
- Select specific files to migrate
- Provide project context to guide the LLM
- Use dry-run mode to preview changes

### How long does migration take?

Migration time depends on:
- Project size (typically 10-30 seconds per file)
- Model speed (local models are slower but free)
- API rate limits (some providers throttle requests)

Typical times:
- **Small project** (5-10 files): 2-5 minutes
- **Medium project** (20-50 files): 10-20 minutes
- **Large project** (100+ files): 30-60+ minutes

## Troubleshooting

### The migration fails with compilation errors

This is normal! py2rs aims for ~70-80% correctness. Common fixes:
1. Check the error report in the log
2. Review the generated Rust code
3. Fix obvious issues (missing imports, type mismatches)
4. Run `cargo check` to identify remaining problems
5. Consider re-running migration with better context/documentation

### The generated code doesn't match Python behavior

AI translations aren't perfect. To improve:
- Add detailed project description before migrating
- Document Python modules with their purpose
- Use a stronger model (GPT-4, Claude Opus)
- Review and manually adjust generated code
- Report issues on GitHub for future improvements

### My API key doesn't work

Check:
- Key format: Anthropic keys start with `sk-ant-`, OpenAI with `sk-`
- Quotas: Ensure your API account has available credits
- Permissions: Some keys are restricted to specific models
- URL: Verify the base URL is correct for your provider

### The app crashes or hangs

Try:
- Restart the application
- Check logs in `~/.py2rs/logs/`
- Reduce thread count in settings (Settings → Thread count → 2)
- Use a smaller model
- Report the issue on GitHub with logs

## Cost & Performance

### How much does migration cost?

Costs vary by provider and project size:

**OpenAI GPT-4**:
- Small project (5 files): ~$0.50-$1.00
- Medium project (20 files): ~$2.00-$5.00
- Large project (100 files): ~$10-$25

**Anthropic Claude**:
- Small project: ~$0.30-$0.80
- Medium project: ~$1.50-$3.50
- Large project: ~$7-$18

**Local models (Ollama)**: Free! But slower.

### Can I pause and resume migration?

Yes! Use the Stop button to halt migration. Files already migrated are saved. You can:
- Deselect completed files
- Run migration again on remaining files
- View history to track previous runs

### How do I reduce costs?

- Use local models (free but slower)
- Migrate only specific files instead of entire project
- Use cheaper models for simple code (GPT-3.5, Claude Haiku)
- Use dry-run mode to test before committing

## Advanced Features

### What is dry-run mode?

Dry-run mode performs migration without writing files to disk. Use it to:
- Preview the migration process
- Test LLM configuration
- Check estimated time/cost
- Verify file selection

### How do .gitignore files work?

When enabled, py2rs respects `.gitignore` patterns and skips:
- Files in `__pycache__/`, `.git/`, `venv/`
- Test files, build artifacts
- Any patterns in your `.gitignore`

### Can I export migration reports?

Yes! After migration completes:
1. Click the "Export" button
2. Choose format: JSON, CSV, or Markdown
3. Save the report for documentation or analysis

Reports include:
- Success/failure status per file
- Compilation errors
- Performance metrics (time, tokens, cost)
- Full file mappings

### How do webhook notifications work?

Configure in Settings → Webhook URL. py2rs will POST migration results:

```json
{
  "project": "/path/to/project",
  "totalFiles": 10,
  "successFiles": 8,
  "failedFiles": 2,
  "duration": 245.6,
  "timestamp": "2026-10-03T15:30:00Z"
}
```

Use this for CI/CD integration or Slack/Discord notifications.

## Contributing

### How can I help improve py2rs?

- Report bugs and issues on GitHub
- Suggest library mappings (Python → Rust crates)
- Contribute code improvements
- Share migration success stories
- Improve documentation

### Where do I report bugs?

Open an issue on GitHub: [https://github.com/YOUR_USERNAME/py2rs/issues](https://github.com/YOUR_USERNAME/py2rs/issues)

Include:
- py2rs version
- Operating system
- LLM provider and model
- Error logs
- Example Python code (if possible)

## Legal & Privacy

### Is my code sent to the cloud?

Only if you use cloud LLM providers (OpenAI, Anthropic, etc.). When using local models via Ollama/llama.cpp, all processing happens on your machine.

### What data is collected?

py2rs stores locally:
- Migration history (in browser localStorage)
- LLM provider settings
- Project paths (not code content)

No telemetry or analytics are sent to external servers.

### Can I use py2rs for commercial projects?

Yes! py2rs is open source (MIT/Apache 2.0 license). You can use it freely for commercial projects. However:
- Check your LLM provider's terms for commercial use
- Review generated code before production deployment
- Maintain proper licensing for dependencies

---

**Still have questions?** Open an issue on [GitHub](https://github.com/YOUR_USERNAME/py2rs/issues) or check the [README](README.md) for more details.
