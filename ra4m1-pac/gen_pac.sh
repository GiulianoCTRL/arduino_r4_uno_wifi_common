#!/bin/sh
PACK_LINK="https://www2.renesas.eu/Keil_MDK_Packs/Renesas.RA_DFP.5.5.0.pack"
PACK_NAME="RA_DFP.zip"
SVD_NAME="R7FA4M1AB.svd"

function setup_svd() {
    curl "$PACK_LINK" -o "$PACK_NAME"
    unzip -jo "$PACK_NAME" "SVD/$SVD_NAME" -d .
    rm "$PACK_NAME"
}

function prepare_workspace() {
    rm -rf src
    mkdir src
}

function generate_pac() {
    svd2rust -i "$SVD_NAME" --target cortex-m
    form -i lib.rs -o src/
    rm lib.rs
    cargo fmt
}

setup_svd
prepare_workspace
generate_pac
