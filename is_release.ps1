function cargo_version {
    $version = Select-String -Path Cargo.toml -pattern  '\d{1,3}.\d{1,3}.\d{1,3}' | Select-Object -First 1
    $version = $version.Line.split('=')[1].trim()
    $version = $version.substring(1, $version.Length-2)
    return $version
}

if ($env:appveyor_repo_tag -eq "true") {
    Add-AppveyorMessage -Message "Tag has been pushed"
    $version = cargo_version
    git tag -a $version -m "$version" 2> $null
    if ($LASTEXITCODE -eq 0) {
    }
    else {
        Add-AppveyorMessage -Message "Tag($version) is already exist"
    }
}
elseif ($env:NEW_TAG -eq "none") {
    $version = cargo_version
    git tag -a $version -m "$version" 2> $null

    if ($LASTEXITCODE -eq 0) {
        Add-AppveyorMessage -Message "Publish new crate version $version"
        # Use AppVeyor API to set variables properly within one build job
        Set-AppveyorBuildVariable -name "NEW_TAG" -Value $version
        Set-AppveyorBuildVariable -Name "APPVEYOR_REPO_TAG_NAME" -Value $version
        Set-AppveyorBuildVariable -Name "APPVEYOR_REPO_TAG" -Value "true"
    }
    else {
        Add-AppveyorMessage -Message "New version publish is not required"
    }
}
# We saved tag name in first build
else {
    Set-AppveyorBuildVariable -Name "APPVEYOR_REPO_TAG_NAME" -Value $env:NEW_TAG
    Set-AppveyorBuildVariable -Name "APPVEYOR_REPO_TAG" -Value "true"
}
