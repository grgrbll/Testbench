import os
import json

def make_ec2_key(name, path):
    key = json.loads(os.popen(f"aws ec2 create-key-pair --key-name \"{name}\"").read())
    pemFilePath = path
    with open(pemFilePath, "w") as pemFile: 
        pemFile.write(key["KeyMaterial"])
    os.system(f"chmod 400 {pemFilePath}")

def ec2_summary():
    summary = {}
    summary["instances"] = json.loads(os.popen("aws ec2 describe-instances").read())
    summary["key-pairs"] = json.loads(os.popen("aws ec2 describe-key-pairs").read())
    return summary

def route53_summary():
    summary = json.loads(os.popen("aws route53 list-hosted-zones").read())
    for zone in summary['HostedZones']:
        zone_id = zone["Id"].split('/')[-1]
        zone["records"] = json.loads(os.popen(f"aws route53 list-resource-record-sets --hosted-zone-id {zone_id}").read())

    return summary

def cloudformation_summary():
    return json.loads(os.popen("aws cloudformation list-stacks").read())

def resources():
    summary = {}
    summary["ec2"] = ec2_summary()
    summary["route53"] = route53_summary()
    summary["cloudformation"] = cloudformation_summary()
    return summary

def deploy_cloudformation():
    pass

def list_projects():
    result = []
    rootdir = os.environ["TESTBENCH_PROJ_ROOT_DIR"]
    for p in os.listdir(rootdir):
        if os.path.isdir(p) and any([x for x in os.listdir(p) if "cloudformation.json" in x]) :
            result.append(os.path.join(rootdir, p))
    return result