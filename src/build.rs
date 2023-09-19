use winres::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_ar_path("x86_64-w64-mingw32-ar")
        .set_windres_path("x86_64-w64-mingw32-windres")
        .set_toolkit_path(".")
        .set_manifest(
            r#"
            <assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1">
                <assemblyIdentity version="1.0.0.0" name="RemapOmen.kabir.kwatra.me"/>
                <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
                    <security>
                        <requestedPrivileges xmlns="urn:schemas-microsoft-com:asm.v3">
                            <requestedExecutionLevel level="asInvoker" uiAccess="true" />
                        </requestedPrivileges>
                    </security>
                </trustInfo>
            </assembly>
        "#,
        )
        .compile()
        .unwrap();
}
