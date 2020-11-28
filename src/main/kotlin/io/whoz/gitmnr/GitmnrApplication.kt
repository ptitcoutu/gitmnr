package io.whoz.gitmnr

import io.quarkus.runtime.QuarkusApplication
import io.quarkus.runtime.annotations.QuarkusMain
import org.eclipse.jgit.api.Git
import picocli.CommandLine
import picocli.CommandLine.IFactory
import java.io.File
import java.nio.file.Files
import javax.inject.Inject


@QuarkusMain
@CommandLine.Command(name = "gitmnr", mixinStandardHelpOptions = true,
        sortOptions = true)
class GitmnrApplication : Runnable, QuarkusApplication {
    @Inject
    lateinit var factory: IFactory

    @CommandLine.Option(names = ["-p", "--project"], required = true, description = ["gradle project name"])
    lateinit var projectName: String

    @CommandLine.Option(names = ["-r", "--rootPath"], required = true, description = ["path of the root folder of the repo"])
    lateinit var repositoryRootFolder: File

    override fun run() {
        println("gitmnr ${projectName} in ${repositoryRootFolder.absolutePath}")
        if (repositoryRootFolder.isDirectory) {
            Git.open(repositoryRootFolder).use { git ->
                git.diff().setOutputStream(System.out).call()
            }
        } else {
            throw Exception("git root folder ${repositoryRootFolder.absolutePath} you provided should be a folder")
        }
    }

    override fun run(vararg args: String?): Int {
        println("args ${args.toList()}")
        return CommandLine(this, factory).execute(*args)
    }
}