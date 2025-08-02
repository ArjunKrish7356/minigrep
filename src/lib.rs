use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use warp::Filter;

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents);
    
    if results.is_empty() {
        println!("No matches found for '{}'", config.query);
    } else {
        println!("Found {} match(es):", results.len());
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub struct Config{
    pub query:String,
    pub filename:String
}

#[derive(Deserialize, Serialize)]
pub struct SearchRequest {
    pub query: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    pub matches: Vec<String>,
    pub count: usize,
}

impl Config{
    pub fn new(args:&[String])->Result<Config,&str>{
        
        if args.len()<3{
            return Err("Not enough arguments");
        }
        
        let query=args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query,filename})
    }
}

pub async fn start_web_server() {
    // Serve static files
    let static_files = warp::path("static")
        .and(warp::fs::dir("static"));
    
    // Serve the main HTML page
    let index = warp::path::end()
        .map(|| warp::reply::html(get_index_html()));
    
    // API endpoint for search
    let search_api = warp::path("api")
        .and(warp::path("search"))
        .and(warp::post())
        .and(warp::body::json())
        .map(|req: SearchRequest| {
            let results = search(&req.query, &req.content);
            let response = SearchResponse {
                count: results.len(),
                matches: results.into_iter().map(|s| s.to_string()).collect(),
            };
            warp::reply::json(&response)
        });
    
    let routes = index.or(search_api).or(static_files);
    
    println!("Starting web server at http://localhost:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn get_index_html() -> &'static str {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MiniGrep - Web Interface</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            text-align: center;
            margin-bottom: 30px;
        }
        .search-form {
            margin-bottom: 20px;
        }
        label {
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
            color: #555;
        }
        input[type="text"], textarea {
            width: 100%;
            padding: 10px;
            border: 2px solid #ddd;
            border-radius: 5px;
            font-size: 16px;
            margin-bottom: 15px;
            box-sizing: border-box;
        }
        textarea {
            height: 200px;
            resize: vertical;
            font-family: monospace;
        }
        button {
            background-color: #007bff;
            color: white;
            padding: 12px 30px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-size: 16px;
            font-weight: bold;
        }
        button:hover {
            background-color: #0056b3;
        }
        button:disabled {
            background-color: #ccc;
            cursor: not-allowed;
        }
        .results {
            margin-top: 20px;
            padding: 20px;
            background-color: #f8f9fa;
            border-radius: 5px;
            border-left: 4px solid #007bff;
        }
        .results h3 {
            margin-top: 0;
            color: #333;
        }
        .match-line {
            background-color: white;
            padding: 8px;
            margin: 5px 0;
            border-radius: 3px;
            font-family: monospace;
            border-left: 3px solid #28a745;
        }
        .no-matches {
            color: #6c757d;
            font-style: italic;
        }
        .error {
            color: #dc3545;
            padding: 10px;
            background-color: #f8d7da;
            border-radius: 5px;
            margin-top: 10px;
        }
        .loading {
            color: #007bff;
            text-align: center;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔍 MiniGrep Web Interface</h1>
        <div class="search-form">
            <form id="searchForm">
                <label for="query">Search Query:</label>
                <input type="text" id="query" name="query" placeholder="Enter text to search for..." required>
                
                <label for="content">Content to Search:</label>
                <textarea id="content" name="content" placeholder="Paste or type the content you want to search in..."></textarea>
                
                <button type="submit" id="searchBtn">Search</button>
            </form>
        </div>
        
        <div id="results" style="display: none;"></div>
    </div>

    <script>
        document.getElementById('searchForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const query = document.getElementById('query').value;
            const content = document.getElementById('content').value;
            const resultsDiv = document.getElementById('results');
            const searchBtn = document.getElementById('searchBtn');
            
            if (!query.trim()) {
                alert('Please enter a search query');
                return;
            }
            
            if (!content.trim()) {
                alert('Please enter content to search');
                return;
            }
            
            // Show loading state
            searchBtn.disabled = true;
            searchBtn.textContent = 'Searching...';
            resultsDiv.innerHTML = '<div class="loading">Searching...</div>';
            resultsDiv.style.display = 'block';
            
            try {
                const response = await fetch('/api/search', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ query, content })
                });
                
                if (!response.ok) {
                    throw new Error('Search failed');
                }
                
                const result = await response.json();
                
                let html = '<h3>Search Results</h3>';
                if (result.count === 0) {
                    html += '<div class="no-matches">No matches found for "' + query + '"</div>';
                } else {
                    html += '<div>Found <strong>' + result.count + '</strong> match(es):</div>';
                    result.matches.forEach(match => {
                        html += '<div class="match-line">' + escapeHtml(match) + '</div>';
                    });
                }
                
                resultsDiv.innerHTML = html;
                
            } catch (error) {
                resultsDiv.innerHTML = '<div class="error">Error: ' + error.message + '</div>';
            } finally {
                searchBtn.disabled = false;
                searchBtn.textContent = 'Search';
            }
        });
        
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        // Load sample content for demonstration
        document.addEventListener('DOMContentLoaded', function() {
            const sampleContent = `Two roads diverged in a yellow wood,
And sorry I could not travel both
And be one traveler, long I stood
And looked down one as far as I could
To where it bent in the undergrowth;

Then took the other, as just as fair,
And having perhaps the better claim,
Because it was grassy and wanted wear;
Though as for that the passing there
Had worn them really about the same,

And both that morning equally lay
In leaves no step had trodden black.
Oh, I kept the first for another day!
Yet knowing how way leads on to way,
I doubted if I should ever come back.

I shall be telling this with a sigh
Somewhere ages and ages hence:
Two roads diverged in a wood, and I—
I took the one less traveled by,
And that has made all the difference.`;
            
            document.getElementById('content').value = sampleContent;
        });
    </script>
</body>
</html>
    "#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![] as Vec<&str>,
            search(query, contents)
        );
    }

    #[test]
    fn no_matches() {
        let query = "xyz";
        let contents = "\
Hello world
This is a test
Nothing here";

        assert_eq!(vec![] as Vec<&str>, search(query, contents));
    }

    #[test]
    fn multiple_matches() {
        let query = "road";
        let contents = "\
Two roads diverged in a yellow wood,
And sorry I could not travel both
Two roads diverged in a wood, and I—";

        assert_eq!(vec!["Two roads diverged in a yellow wood,", "Two roads diverged in a wood, and I—"], search(query, contents));
    }
}