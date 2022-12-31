
import time
from tqdm import tqdm

import os
import glob
import subprocess
from src.utils.build_utils import can_compile
from src.errors import UnknownOutputType
from src.logger import *

from src.helpers import assert_dict

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
    include_args = []

    for include in config["includes"]:
        logger.verbose("Adding header folder/file (%s)" % include)
        include_args.append("-I%s" % include)

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

                if can_compile(file, full_path):

                    pbar.set_description("[%s]: Compiling %s" % (logger.prefix.info, file))

                    command = []

                    command.append(config["compiler"])
                    command += config["compiler_flags"]

                    if config["output_type"] == "lib":
                        command.append("-shared")
                        command.append("-fPIC")

                    command += include_args
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

    logger.verbose("Linker flags: %s" % config["linker_flags"])
    logger.verbose("Linking object files (%s) to '%s'" % (config["compiled_objects"], config['output']))

    command = []

    command.append(config["compiler"])
    command += config["linker_flags"]
    command += config["compiled_objects"]
    command.append("-o")
    command.append(config['output'])

    subprocess.run(command, check=True)
    logger.info(f"Linked object files ({config['output']})")

def emit_library(config):

    logger.verbose("Linker flags: %s" % config["linker_flags"])
    logger.verbose("Linking object files (%s) to '%s'" % (config["compiled_objects"], config['output']))

    command = []

    command.append(config["compiler"])
    command.append("-shared")
    command.append("-fPIC")
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
        "includes": [],
        "source_files": [],
        "compiled_objects": [],
        "breeze_folder": config[".folder"]
    }

    logger.verbose("Fetching necesary information")

    assert_dict(config, "config")
    assert_dict(config["config"], "cpp-lang")

    cpp_lang_config = config["config"]["cpp-lang"]

    assert_dict(cpp_lang_config, "sources", "config.cpp-lang.sources")
    assert_dict(cpp_lang_config, "type", "config.cpp-lang.type")

    compiler = cpp_lang_config.get("compiler", None)
    if compiler is None:
        compiler = find_default_compiler()

    output_type = cpp_lang_config["type"]

    create_cpp_folder(build_config)

    build_config["compiler"] = compiler
    build_config["output_type"] = output_type
    build_config["project_name"] = config["project"]["name"]

    build_config["linker_flags"] = cpp_lang_config.get("linker_flags", "")
    build_config["compiler_flags"] = cpp_lang_config.get("compiler_flags", "")

    logger.verbose("Retrieving source files")

    if isinstance(cpp_lang_config["sources"], str):
        build_config["source_files"] += glob.glob(cpp_lang_config["sources"], recursive=True)
    elif isinstance(cpp_lang_config["sources"], list):
        for source in cpp_lang_config["sources"]:
            build_config["source_files"] += glob.glob(source, recursive=True)
    else:
        pass # TODO: error?

    logger.verbose("Retrieving header files")

    if cpp_lang_config.get("include", None) is not None:
        if isinstance(cpp_lang_config["include"], str):
            build_config["includes"].append(cpp_lang_config["include"])
        elif isinstance(cpp_lang_config["include"], list):
            for source in cpp_lang_config["include"]:
                build_config["includes"].append(source, recursive=True)
        else:
            pass # TODO: error?

    compile_project(build_config)

    if output_type == "exec":
        logger.verbose("Compiling executable...")
        build_config["output"] = cpp_lang_config.get("output", "a.out")
        link_objects(build_config)
    elif output_type == "lib":
        logger.verbose("Compiling library...")
        build_config["output"] = cpp_lang_config.get("output", "a.so")
        emit_library(build_config)
    else:
        raise UnknownOutputType(f"Output type '{output_type}' is not supported")
