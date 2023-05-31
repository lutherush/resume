# Building My Resume

## Overview

I'm maintaining the contents of my resume in the [`resume.yaml`](resume.yaml)
file. The [`resume.py`](resume.py) script builds several versions from Jinja2
templates:

| Document                | Template
| ----------------------- | -----------------------------------------
| HTML                    | [`templates/resume.html`](templates/resume.html)
| Markdown                | [`templates/resume.md`](templates/resume.md)
| Gemini                  | [`templates/resume.gmi`](templates/resume.gmi)
| Plain Text              | [`templates/resume.txt`](templates/resume.txt)
| Plain Text Narrow Width | [`templates/resume-narrow.txt`](templates/resume-narrow.txt)

It also produces a JSON file directly from the YAMl source (not a template).

The [`dist/`](dist) directory contains assets that are deployed to my resume
site as-is - currently, that's just a few images. The `dist` directory acts as a
staging directory - the generated `index.html` would exist at the root of this.

I deploy to S3 and my local Gemini and Gopher servers.

## Files

```plain
├── .github/                GitHub Workflows
├── BUILDING.md             This document that describes how it's built and deployed
├── README.md               A Markdown document generated and merged via CI (GitHub action) or via the `resume.py` script
├── dist/                   Directory containing things to deploy
│   └── assets/             Static assets for the website
│       └── img/
├── Makefile                Makefile for building and local tasks
├── requirements.txt        Python dependencies
├── resume.py               Python script for building HTML and Markdown
├── resume.yaml             My resume data
└── templates/              Source template files
```

### Other Files

Some other files for various integrations are also maintained in this repository:

* `.codacy.yml`: configuration for Codacy (via GitHub)
* `.deepsource.toml`: configuration for DeepSource integration (via GitHub)
* `.remarkrc.js`: configuration for the Remark Markdown validator (via Codacy, CodeFactor)
* `.stylelintrc.json`: configuration for linters (used via GitHub - Codacy, CodeFactor)
* `renovate.json`: configuration for RenovateBot (via GitHub)

## Build and Deployment

Use the [`Makefile`](Makefile) to build the resume.

To build all formats:

```shell
make
```

To build HTML only:

```shell
make html
```

The GitHub Workflow does several things:

* Spell check (the contents of the most recent commit)
* Builds HTML and Markdown using [`resume.py`](resume.py)
* Generates Word document using [Pandoc](https://pandoc.org/)
* Generates PDF document using [GitHub action for headless Chrome](https://github.com/marketplace/actions/setup-chrome)

My resume is published to my webside. My website is simple static page hosted with Hugo on my private kubernetes cluster. 
My cluster runs on 3 nodes in my living room and uses digitalOcean for external proxy. For nodes i use Intel nuc extreme.
In addition i have 4 proxmox server in test lab and 34TB FreeNAS server. ( at moment my web is down due to home relocation)
Resume versions are hosted in local minio serving as private s3 bucket. In upcomming days i will upload all my IaC files that are used 
to deploy my cluster and its config, build and deploy my page and host my cv.

It's also useful for local development to preview the site with
an actual HTTP server rather than as a local file, which is helpful for testing
relative URLs and the like similar to 'production'.

```shell
make serve
```

Visit the local instance via <http://localhost:8080/resume/>

## Why

Just for fun. I've maintained it in different ways over the years, more recently in simple HTML. However, I'd like to be able to
provide it in Markdown, too, and also other formats as desired. YAML seems like the best choice for a human-friendly and
machine-parsable format to maintain the source in to produce multiple formats. While this may not be the most intuitive for many people,
it's all comfortable to me. Also, i am not big fan of resumes as they usually contain buzzwords, i rather show you what i can do.

