# Placeholder Image generator

https://placeholderimg.net/

## File and directory structure overview

- generator - Lambda backend function to generate the image
- frontend - Frontend web file
- scripts - Root npm scripts
- bin, lib, files in root directory - AWS CDK deploy stack

## How to build your copy of the site

### Prerequiste
- Typescript
- AWS CDK 1.85 or later
- Rust musl target for lambda function binary 


    rustup target add x86_64-unknown-linux-musl

### Download 

    git clone [this repo] --recursive
    # or, 'git submodule update --init' if you've already cloned.

### Configure    

TODO: write here.

See and edit frontend/index.html URLs.

See lib/cdk-config.ts 

### Build & Deploy
In root directory run

    npm run build 
    cdk deploy

