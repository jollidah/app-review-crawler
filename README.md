# App Review Crawler

A high-performance, asynchronous Rust application for crawling app reviews from both the Apple App Store and Google Play Store. This tool helps developers and researchers collect and analyze user feedback from multiple app stores efficiently.

## 🌟 Features

- **Multi-Store Support**: Crawl reviews from both Apple App Store and Google Play Store
- **Asynchronous Processing**: Built with Tokio for high-performance concurrent crawling
- **Pagination Support**: Automatically handles multiple pages of reviews (up to 10 pages for App Store, 100 for Play Store)
- **CSV Export**: Saves reviews in structured CSV format for easy analysis
- **Configurable**: Easy configuration through JSON files
- **Error Handling**: Robust error handling with detailed logging
- **Rate Limiting**: Built-in delays to respect API limits

> **⚠️ Note**: Play Store crawling is currently a placeholder implementation and not fully functional. Only App Store crawling is fully implemented and tested.

## 📊 Current Status

| Feature | App Store | Play Store |
|---------|-----------|------------|
| Review Crawling | ✅ Fully Working | ❌ Placeholder Only |
| Pagination | ✅ Up to 10 pages | ❌ Not Implemented |
| CSV Export | ✅ Working | ❌ Not Implemented |
| Error Handling | ✅ Robust | ❌ Basic |
| Rate Limiting | ✅ Implemented | ❌ Not Implemented |

**Currently, only App Store review crawling is fully functional and tested.**

## 📋 Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo package manager

## 🚀 Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/app-review-crawler.git
   cd app-review-crawler
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the application**:
   ```bash
   cargo run --release
   ```

## 📥 Downloads

Grab the latest release binaries for your platform:

- **macOS (aarch64)**  
  [app-review-crawler-aarch64-apple-darwin.tar.gz](https://github.com/jollidah/app-review-crawler/releases/download/release%2Fv0.0.1/app-review-crawler-v0.0.1-aarch64-apple-darwin.tar.gz)

- **macOS (x86_64)**  
  [app-review-crawler-x86_64-apple-darwin.tar.gz](https://github.com/jollidah/app-review-crawler/releases/download/release%2Fv0.0.1/app-review-crawler-v0.0.1-x86_64-apple-darwin.tar.gz)

- **Windows (x86_64)**  
  [app-review-crawler-x86_64-pc-windows-msvc.zip](https://github.com/jollidah/app-review-crawler/releases/download/release%2Fv0.0.1/app-review-crawler-v0.0.1-x86_64-pc-windows-msvc.zip)

- **Linux (x86_64)**  
  [app-review-crawler-x86_64-unknown-linux-gnu.tar.gz](https://github.com/jollidah/app-review-crawler/releases/download/release%2Fv0.0.1/app-review-crawler-v0.0.1-x86_64-unknown-linux-gnu.tar.gz)

### Quick Start with Binary

1. **Download** the appropriate binary for your platform
2. **Extract** the archive:
   ```bash
   # macOS/Linux
   tar -xzf app-review-crawler-v0.0.1-*.tar.gz
   
   # Windows
   # Extract the .zip file using your preferred tool
   ```
3. **Make executable** (macOS/Linux):
   ```bash
   chmod +x app-review-crawler
   ```
4. **Run** the application:
   ```bash
   ./app-review-crawler
   ```

## 📋 Project Structure

```
app-review-crawler/
├── src/
│   ├── main.rs                 # Main application entry point
│   ├── errors.rs               # Error handling definitions
│   ├── target_app.rs           # Target app configuration loading
│   ├── review_crawler/         # Crawling logic
│   │   ├── mod.rs             # Crawler implementation
│   │   ├── app_store.rs       # App Store specific crawler
│   │   ├── play_store.rs      # Play Store specific crawler
│   │   └── traits.rs          # Common traits for crawlers
│   └── response_processor/     # Response processing and CSV export
│       ├── mod.rs             # Response processor implementation
│       ├── app_store.rs       # App Store review parsing
│       ├── play_store.rs      # Play Store review parsing
│       └── traits.rs          # Processing traits
├── target_apps.json           # Configuration file for target apps
├── output/                    # Generated CSV files (auto-created)
├── Cargo.toml                 # Rust dependencies
└── README.md                  # This file
```

## ⚙️ Configuration

Create a `target_apps.json` file in the project root to specify which apps to crawl:

```json
{
  "app_store_apps": [
    {
      "app_id": "1194408342",
      "country": "us",
      "pages": 0
    },
    {
      "app_id": "284882215",
      "country": "kr",
      "pages": 0
    }
  ],
  "play_store_apps": [
    {
      "app_id": "com.whatsapp",
      "country": "us",
      "pages": 0
    },
    {
      "app_id": "com.instagram.android",
      "country": "kr",
      "pages": 0
    }
  ]
}
```

> **⚠️ Note**: The `play_store_apps` section is included for future compatibility, but Play Store crawling is not yet implemented. Only App Store apps will be processed.

### Configuration Fields

- **app_id**: The unique identifier for the app
  - App Store: Numeric ID (e.g., "1194408342")
  - Play Store: Package name (e.g., "com.whatsapp")
- **country**: Two-letter country code (e.g., "us", "kr", "jp")
- **pages**: Starting page number (usually 0 or 1)

## 📊 Output Format

Reviews are saved as CSV files in the `output/` directory:

### App Store Reviews (`output/app_store/{app_id}.csv`)
```csv
date,star,like,dislike,title,review
2025-05-11T10:19:38-07:00,2,0,0,"Great idea but not well executed.","If you are test, this isn't it..."
2025-03-30T15:13:14-07:00,4,0,0,"Love it!!","Super helpful and cute!..."
```

### Play Store Reviews (`output/play_store/{app_id}.csv`)
```csv
date,star,like,dislike,title,review
2025-01-15T12:30:00Z,5,10,2,"Amazing app!","This app is fantastic..."
```

> **⚠️ Note**: Play Store CSV files are not currently generated as the Play Store crawling functionality is not yet implemented.

## 🔧 Usage Examples

### Basic Usage

1. **Configure your target apps**:
   ```bash
   # Edit target_apps.json with your desired apps
   nano target_apps.json
   ```

2. **Run the crawler**:
   ```bash
   cargo run --release
   ```

3. **Check the results**:
   ```bash
   ls output/app_store/
   ls output/play_store/
   ```

### Programmatic Usage

```rust
use app_review_crawler::{
    review_crawler::{Crawler, app_store::AppStoreClient},
    response_processor::{ResponseProcessor, RawResponse, app_store::AppStoreReview},
};

#[tokio::main]
async fn main() {
    // Create a crawler for a specific app
    let mut client = AppStoreClient {
        app_id: "1194408342".to_string(),
        country: "us".to_string(),
        pages: 0,
    };
    
    let mut crawler = Crawler::new(client);
    
    // Crawl reviews
    match crawler.run().await {
        Ok(responses) => {
            let processor = ResponseProcessor::new(
                RawResponse::new(responses),
                AppStoreReview::new(),
                "1194408342".to_string()
            );
            
            if let Err(e) = processor.run().await {
                tracing::error!("Failed to process reviews: {}", e);
            }
        }
        Err(e) => tracing::error!("Failed to crawl: {}", e),
    }
}
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_extract_app_store_reviews

# Run tests with output
cargo test -- --nocapture
```

## 📝 Logging

The application provides detailed logging:

- `[INFO]`: General information about the crawling process
- `[DEBUG]`: Detailed debugging information
- `[ERROR]`: Error messages and failures

## 🤝 Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This tool is for educational and research purposes. Please ensure you comply with:

- Apple's Terms of Service for App Store data
- Google's Terms of Service for Play Store data
- Respect rate limits and robots.txt files
- Use responsibly and ethically

## 🐛 Known Issues

- **Play Store Implementation**: Play Store crawling is currently a placeholder and not functional. Only App Store crawling works.
- Some Unicode characters in reviews may cause parsing issues
- Rate limiting may need adjustment based on server response

## 🔮 Roadmap

- [ ] **Complete Play Store implementation** (Currently placeholder only)
- [ ] Add support for more app stores
- [ ] Implement review filtering and search
- [ ] Add database storage option
- [ ] Create web dashboard
- [ ] Add review sentiment analysis
- [ ] Support for review replies and developer responses

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/app-review-crawler/issues) page
2. Create a new issue with detailed information
3. Include your configuration and error logs

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Async runtime powered by [Tokio](https://tokio.rs/)
- HTTP client by [reqwest](https://github.com/seanmonstar/reqwest)
- XML parsing with [quick-xml](https://github.com/tafia/quick-xml)

---

**Written by Rust**
