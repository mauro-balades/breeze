
from tqdm import tqdm

import os
import glob
import subprocess
from cli.errors import UnknownOutputType
from cli.logger import *

from cli.helpers import assert_dict

def find_default_compiler():
    # TODO: https://cmake.org/pipermail/cmake/2013-March/053819.html
    return "g++"

def create_cpp_folder(build_config):
    def create_dir(dir: str):
        if not os.path.exists(dir):
            os.mkdir(dir)

    cpp_folder = os.path.join(build_config["breeze_folder"], ".cpp")
    create_dir(cpp_folder)

    cpp_folder = os.path.join(cpp_folder, build_config["project_name"])
    create_dir(cpp_folder)

    create_dir(os.path.join(cpp_folder, "cache"))
    create_dir(os.path.join(cpp_folder, "config"))
    create_dir(os.path.join(cpp_folder, "files"))

def compile_project(config):
    with tqdm(total=len(config["source_files"]), leave=False) as pbar:
        for file in config["source_files"]:
            if os.path.isfile(file):
                full_folder = os.path.join(config["breeze_folder"], ".cpp", "files")
                for folder in file.split(os.path.sep)[:-1]:
                    if folder == ".": continue

                    new_folder = os.path.join(full_folder, folder + ".dir")
                    full_folder = new_folder

                    if not os.path.exists(new_folder):
                        os.mkdir(new_folder)

                full_path = os.path.join(os.getcwd(), full_folder, os.path.basename(file) + ".o")
                config["compiled_objects"].append(full_path)

                pbar.set_description("[%s]: Compiling %s" % (logger.prefix.info, file))

                command = []

                command.append(config["compiler"])
                command += config["compiler_flags"]

                if config["output_type"] == "lib":
                    command.append("-shared")
                    command.append("-fPIC")

                command.append("-c")
                command.append(os.path.join(os.getcwd(), file))
                command.append("-o")
                command.append(full_path)

                subprocess.run(command, check=True)

                pbar.update(1)

            else:
                raise Exception(f"'{file}' is not a file!")

        pbar.clear()
        logger.info("Compiled object files")

def link_objects(config):
    command = []

    command.append(config["compiler"])
    command += config["linker_flags"]
    command += config["compiled_objects"]
    command.append("-o")
    command.append(config['output'])

    subprocess.run(command, check=True)
    logger.info(f"Linked object files ({config['output']})")

def emit_library(config):
    command = []

    command.append(config["compiler"])
    command.append("-shared")
    command += config["linker_flags"]
    command += config["compiled_objects"]
    command.append("-o")
    command.append(config['output'])

    subprocess.run(command, check=True)
    logger.info(f"Linked object files ({config['output']})")

def build(config):

    build_config = {
        "compiler": "",
        "linker_flags": "",
        "compiler_flags": "",
        "project_name": "",
        "output": "",
        "output_type": "",
        "source_files": [],
        "compiled_objects": [],
        "breeze_folder": config[".folder"]
    }

    assert_dict(config, "config")
    assert_dict(config["config"], "cpp-lang")

    cpp_lang_config = config["config"]["cpp-lang"]

    assert_dict(cpp_lang_config, "sources", "config.cpp-lang.sources")
    assert_dict(cpp_lang_config, "type", "config.cpp-lang.type")

    compiler = cpp_lang_config.get("compiler", None)
    if compiler is None:
        compiler = find_default_compiler()

    output_type = cpp_lang_config["type"]
    if (output_type != "exec" and
        output_type != "lib"):
        raise UnknownOutputType(f"Output type '{output_type}' is not supported")

    create_cpp_folder(build_config)

    build_config["compiler"] = compiler
    build_config["output_type"] = output_type
    build_config["project_name"] = config["project"]["name"]

    if isinstance(cpp_lang_config["sources"], str):
        build_config["source_files"] += glob.glob(cpp_lang_config["sources"], recursive=True)
    elif isinstance(cpp_lang_config["sources"], list):
        for source in cpp_lang_config["sources"]:
            build_config["source_files"] += glob.glob(source, recursive=True)
    else:
        pass # TODO: error?

    compile_project(build_config)

    if output_type == "exec":
        build_config["output"] = cpp_lang_config.get("output", "a.out")
        link_objects(build_config)
    elif output_type == "lib":
        build_config["output"] = cpp_lang_config.get("output", "a.so")
        emit_library(build_config)
