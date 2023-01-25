import { spawn } from "child_process"

export const spawnAsync = async (command: string, options: Array<string>) => {
    const p = spawn(command, options)

    const stdout = new Array<string>()
    const stderr = new Array<string>()

    try {
        await new Promise((resolve) => {
            p.stdout.on("data", (x) => {
                process.stdout.write(x.toString())
            })
            p.stderr.on("data", (x) => {
                process.stderr.write(x.toString())
            })
            p.on("exit", (code) => {
                resolve(code)
            })
        })
    }
    catch (e) {
        if (e instanceof Error) {
            stderr.push("\n"+e.message+"\n"+e.stack+"\n")
        }
        else {
            stderr.push("\n"+String(e)+"\n")
        }
    }

    return {
        stdout: stdout.join(""),
        stderr: stderr.join(""),
    }
}
