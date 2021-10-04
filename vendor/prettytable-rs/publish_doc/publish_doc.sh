#!/bin/sh

CRATE_NAME="$1"
MASTER_DOC_ONLY="$2"
PUBLISH_CRATE="no"
PUBLISH_DOC="no"
if [ -z "$CRATE_NAME" ]; then
    echo "Please provide crate name as first argument"
    exit 1
fi

if [ -z "${GH_TOKEN}" ] ; then
    echo "GH_TOKEN environment variable is not set. exiting ..."
    exit 1
fi
[ "$TRAVIS_RUST_VERSION" = "stable" ] || exit 0
[ "$TRAVIS_PULL_REQUEST" = false ] || exit 0
if [ "$TRAVIS_BRANCH" = "master" ]; then
    RELEASE="master"
    PUBLISH_DOC="yes"
    COMMIT_COMMENT="Updated documentation on master for ${TRAVIS_COMMIT}"
elif [ ! -z "$TRAVIS_TAG" ]; then
    echo "$TRAVIS_TAG" | grep -Eq "^v[0-9]+\.[0-9]+\.[0-9]+$" || exit 1
    RELEASE=$TRAVIS_TAG
    PUBLISH_CRATE="yes"
    [ "$MASTER_DOC_ONLY" != "yes" ] && PUBLISH_DOC="yes"
    COMMIT_COMMENT="Updated documentation for release $RELEASE"
else
    exit 1
fi

if [ "$PUBLISH_DOC" = "yes" ]; then
    echo "Publishing documentation for $RELEASE"

    cargo doc || exit 1
    if [ ! -d "target/doc/$CRATE_NAME" ]; then
        echo "Cannot find target/doc/$CRATE_NAME"
        exit 1
    fi

    git clone --depth=50 --branch=gh-pages "https://github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
    rm -Rf "gh-pages/${RELEASE}"
    mkdir "gh-pages/${RELEASE}"

    echo "<meta http-equiv=refresh content=0;url=${CRATE_NAME}/index.html>" > target/doc/index.html
    cp -R target/doc/* "gh-pages/${RELEASE}/"

    INDEX="gh-pages/index.html"
    echo "<html><head><title>${TRAVIS_REPO_SLUG} API documentation</title></head><body>" > $INDEX
    echo "<h1>API documentation for crate <a href='https://github.com/${TRAVIS_REPO_SLUG}'>${CRATE_NAME}</a></h1>" >> $INDEX
    echo "<strong>Select a crate version :</strong><br/>" >> $INDEX
    for entry in $(ls -1 gh-pages)
    do
        [ -d "gh-pages/$entry" ] && echo "<a href='${entry}/index.html'>${entry}</a><br/>" >> $INDEX
    done
    echo "</body></html>" >> $INDEX

    cd gh-pages || exit 1
    git config user.name "travis-ci"
    git config user.email "travis@travis-ci.org"

    git add --all
    git commit -m "$COMMIT_COMMENT"

    git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages || exit 1
    cd ..
    rm -Rf "gh-pages"
fi

if [ "$PUBLISH_CRATE" = "yes" ] && [ ! -z "$CRATES_IO_TOKEN" ]; then
    # Check version in Cargo.toml
    version=$(egrep "^version = " Cargo.toml | sed 's/version \+= \+"\([0-9]\+\.[0-9]\+\.[0-9]\)"/\1/' | tr -d "\r\n")
    tag_version=$(echo "$RELEASE" | sed 's/v\([0-9]\+\.[0-9]\+\.[0-9]\+\)/\1/')
    if [ "$version" != "$tag_version" ]; then
        echo "ERROR : Version in Cargo.toml ($version) differs from $tag_version"
        exit 1
    fi
    echo "Publishing $RELEASE to crates.io"
    cargo login "$CRATES_IO_TOKEN" || exit 1
    cargo publish || exit 1
fi