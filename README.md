# project-selector
Select Projects

# Build
Move into the directory and run
```bash
cargo build --release
sudo mv target/release/project-selector /usr/bin
```

# Configure
In config folder make a file called "projects.json"

The config will look like this:
```json
{
  "dir": "/path_to_project_directory",
  "term": "/path_to_preffered_terminal"
}
```

# Launch
To launch run
```
project-selector $PPID
```

I personally prefer to put this in an alias
