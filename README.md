## Script to check what repos havo no readmes  

to run use this, needs github-cli:
`gh repo list --no-archived --source --limit 999  | cargo run`