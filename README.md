# Maybe Unstable

Refresh input:

`(Invoke-WebRequest -UseBasicParsing -URI https://rss.nytimes.com/services/xml/rss/nyt/recent.xml).Content | Out-File '.\sample input.xml' -Encoding 'utf8'`
