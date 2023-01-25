import { Result, Option } from "@frank-mayer/opsult"
import decompress from "decompress"
import { existsSync } from "fs"
import { readFile, writeFile, rm, readdir } from "fs/promises"
import fetch from "node-fetch"
import { join } from "path"

type GitHubAsset = {
    id: number;
    name: string;
    content_type: string;
    browser_download_url: string;
}

type GitHubApiRelease = {
    id: number;
    name: string;
    tag_name: string;
    assets: Array<GitHubAsset>
}

enum system {
    winArm64 = "windows-arm64",
    win64 = "windows-amd64",
    win32 = "windows-386",
    linuxArm64 = "linux-arm64",
    linux64 = "linux-amd64",
    linux32 = "linux-386",
    macArm64 = "darwin-arm64",
    mac64 = "darwin-amd64",
    mac32 = "darwin-386",
}

const wakatimeVersionFile = "./wakatimeversion.txt"
const wakatimeCliFolder = "./wakatime-cli"

const getSystem = (): Option<system> => {
    const arch = process.arch
    const platform = process.platform
    if (platform === "win32") {
        if (arch === "x64") {
            return Option.Some(system.win64)
        } if (arch === "arm64") {
            return Option.Some(system.winArm64)
        } 
        return Option.Some(system.win32)
    }

    if (platform === "linux") {
        if (arch === "x64") {
            return Option.Some(system.linux64)
        } if (arch === "arm64") {
            return Option.Some(system.linuxArm64)
        } 
        return Option.Some(system.linux32)
    }
    
    if (platform === "darwin") {
        if (arch === "x64") {
            return Option.Some(system.mac64)
        } if (arch === "arm64") {
            return Option.Some(system.macArm64)
        } 
        return Option.Some(system.mac32)
    }

    return Option.None()
}

const getCurrentWakatimeVersionAsync = async () => {
    if (!existsSync(wakatimeVersionFile)) {
        return "v0.0.0"
    }
    
    const version = await readFile("wakatimeversion.txt", "utf-8")
    return version
}

export const getLatestWakatimeAsync = async (): Promise<Result<string, string>> => {
    const response = await fetch("https://api.github.com/repos/wakatime/wakatime-cli/releases/latest")
    if (!response.ok) {
        Result.Err("Failed to fetch latest wakatime version")
    }

    const json = await response.json() as GitHubApiRelease
    const newVersion = json.name
    const currentVersion = await getCurrentWakatimeVersionAsync()
    if (newVersion === currentVersion) {
        return Result.Ok(newVersion)
    }

    const system = getSystem()
    if (system.isNone) {
        return Result.Err("Unsupported system")
    }

    const asset = json.assets
        .find((a) => a.name.includes(system.unwrap()))
    
    if (!asset) {
        return Result.Err("No binary found for this system")
    }

    const downloadResponse = await fetch(asset.browser_download_url)
    if (!downloadResponse.ok) {
        return Result.Err("Failed to download wakatime binary")
    }

    const blob = await downloadResponse.blob()
    const buffer = Buffer.from(await blob.arrayBuffer())
    await writeFile(wakatimeCliFolder + ".zip", buffer, "binary")

    try {
        await Result.WrapPromise(decompress(wakatimeCliFolder + ".zip", wakatimeCliFolder))
    }
    catch (e) {
        return Result.Err("Failed to unzip wakatime binary")
    }
    finally {
        await rm(wakatimeCliFolder + ".zip")
    }

    await writeFile(wakatimeVersionFile, newVersion, "utf-8")

    return Result.Ok(newVersion)
}

export const getWakatimeExePathAsync = async (): Promise<Result<string, string>> => {
    const dir = await readdir(wakatimeCliFolder)
    
    switch (dir.length) {
    case 0:
        return Result.Err("No wakatime binary found")
    case 1:
        return Result.Ok(join(wakatimeCliFolder, dir[0]))
    default:
    {
        const exe = dir.find((d) => d.includes("wakatime-cli"))
        if (!exe) {
            return Result.Err("No wakatime binary found, multiple files but no wakatime-cli")
        }

        return Result.Ok(join(wakatimeCliFolder, exe))
    }
    }
}
