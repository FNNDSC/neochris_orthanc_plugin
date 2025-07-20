# Generate required Rust code
codegen: generate-bindings generate-client

# Generate Orthanc model files using OpenAPI.
generate-client: && _postprocess_client
    openapi-generator-cli batch --clean --fail-fast --threads 1 openapi-generator.yaml

# Post-processing of openapi-generator-cli output.
_postprocess_client: (_fix_rustdoc_urls 'orthanc_ogen_client/src/models') _overlay_client

# Overwrite openapi-generator-cli created files with manually specified content.
_overlay_client:
    cp -rfv orthanc_client_ogen_overlay/* orthanc_client_ogen

# Surround URLs in Rust documentation with `<`, `>` characters.
_fix_rustdoc_urls dir:
    fd --no-ignore-vcs --type f --exact-depth 1 --extension .rs '{{dir}}' \
       --exec sed -ie 's#\(///.*: \)\(https://[a-zA-Z0-9\./]*\)#\1<\2>#' '{}'

# Generate Orthanc SDK bindings using bindgen
generate-bindings: mkdir-3rdparty
    wget -O 3rdparty/OrthancCPlugin.h 'https://orthanc.uclouvain.be/hg/orthanc/raw-file/Orthanc-1.12.8/OrthancServer/Plugins/Include/orthanc/OrthancCPlugin.h'
    bindgen 3rdparty/OrthancCPlugin.h -o src/bindings.rs

# Create 3rdparty directory
mkdir-3rdparty:
    mkdir -p 3rdparty
    echo '*' > 3rdparty/.gitignore

