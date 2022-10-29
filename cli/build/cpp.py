
import os
import glob

from cli.helpers import assert_dict

def find_default_compiler():
    # TODO: https://cmake.org/pipermail/cmake/2013-March/053819.html
    return "g++"

def create_cpp_folder(build_config):
    cpp_folder = os.path.join(build_config["breeze_folder"], ".cpp")
    if not os.path.exists(cpp_folder):
        os.mkdir(cpp_folder)

    cpp_folder = os.path.join(cpp_folder, build_config["project_name"])
    if not os.path.exists(cpp_folder):
        os.mkdir(cpp_folder)

def build(config):

    build_config = {
        "compiler": "",
        "linker_flags": "",
        "compiler_flags": "",
        "project_name": "",
        "source_files": [],
        "breeze_folder": config[".folder"]
    }

    assert_dict(config, "config")
    assert_dict(config["config"], "cpp-lang")

    cpp_lang_config = config["config"]["cpp-lang"]

    assert_dict(cpp_lang_config, "sources", "config.cpp-lang.sources")

    compiler = cpp_lang_config.get("compiler", None)
    if compiler is None:
        compiler = find_default_compiler()

    create_cpp_folder(build_config)

    build_config["compiler"] = compiler
    build_config["project_name"] = config["project"]["name"]

    if isinstance(cpp_lang_config["sources"], str):
        build_config["source_files"] += glob.glob(cpp_lang_config["sources"], recursive=True)
    elif isinstance(cpp_lang_config["sources"], list):
        for source in cpp_lang_config["sources"]:
            build_config["source_files"] += glob.glob(source, recursive=True)
    else:
        pass # TODO: error?

    print(build_config)
    print(config)
