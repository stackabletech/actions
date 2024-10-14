import importlib.util
import sys
import os

sys.path.append(str(os.getcwd()))

def import_from_path(module_name, file_path):
    spec = importlib.util.spec_from_file_location(module_name, file_path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)
    return module

# import smoke.conf as conf
file = os.environ['CONFIG_FILE'] if os.environ['CONFIG_FILE'] else 'conf'
conf = import_from_path('conf', file)

product=os.environ['PRODUCT_NAME']
print(f"Generating version list for {product}")

# get the product config
product_conf = list(filter(lambda x: x["name"] == product, conf.products))[0]
# list the versions, eg: [1.0, 1.1, 2.0]
versions = [v["product"] for k,v in enumerate(product_conf["versions"])]
output_versions = f"VERSIONS={versions}\n"

github_outputs_file = os.environ['GITHUB_OUTPUT']
f = open(github_outputs_file, "w")
print(f"Writing to $GITHUB_OUTPUT: {output_versions}")
f.write(output_versions)
f.close()
