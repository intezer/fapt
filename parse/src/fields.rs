/// GENERATED by gen.py; do not edit

use apt_capnp::source;
use errors::*;
use blank_to_null;

pub const HANDLED_FIELDS: [&'static str; 29] = [
    "Architecture",
    "Binary",
    "Build-Conflicts",
    "Build-Conflicts-Arch",
    "Build-Conflicts-Indep",
    "Build-Depends",
    "Build-Depends-Arch",
    "Build-Depends-Indep",
    "Checksums-Md5",
    "Checksums-Sha1",
    "Checksums-Sha256",
    "Checksums-Sha512",
    "Files",
    "Format",
    "Package",
    "Package-List",
    "Priority",
    "Source",
    "Vcs-Arch",
    "Vcs-Browse",
    "Vcs-Browser",
    "Vcs-Bzr",
    "Vcs-Cvs",
    "Vcs-Darcs",
    "Vcs-Git",
    "Vcs-Hg",
    "Vcs-Mtn",
    "Vcs-Svn",
    "Version",

];

pub fn set_field(key: &str, val: &str, builder: &mut source::Builder) -> Result<()> {
    match key {
        "Autobuild" => blank_to_null(val, |x| builder.set_autobuild(x)),
        "Breaks" => blank_to_null(val, |x| builder.set_breaks(x)),
        "Bugs" => blank_to_null(val, |x| builder.set_bugs(x)),
        "Build-Indep-Architecture" => blank_to_null(val, |x| builder.set_build_indep_architecture(x)),
        "Built-For-Profiles" => blank_to_null(val, |x| builder.set_built_for_profiles(x)),
        "Built-Using" => blank_to_null(val, |x| builder.set_built_using(x)),
        "Class" => blank_to_null(val, |x| builder.set_class(x)),
        "Conffiles" => blank_to_null(val, |x| builder.set_conffiles(x)),
        "Config-Version" => blank_to_null(val, |x| builder.set_config_version(x)),
        "Conflicts" => blank_to_null(val, |x| builder.set_conflicts(x)),
        "Debian-Vcs-Browser" => blank_to_null(val, |x| builder.set_debian_vcs_browser(x)),
        "Debian-Vcs-Git" => blank_to_null(val, |x| builder.set_debian_vcs_git(x)),
        "Debian-Vcs-Svn" => blank_to_null(val, |x| builder.set_debian_vcs_svn(x)),
        "Depends" => blank_to_null(val, |x| builder.set_depends(x)),
        "Description" => blank_to_null(val, |x| builder.set_description(x)),
        "Description-md5" => blank_to_null(val, |x| builder.set_description_md5(x)),
        "Dgit" => blank_to_null(val, |x| builder.set_dgit(x)),
        "Directory" => blank_to_null(val, |x| builder.set_directory(x)),
        "Dm-Upload-Allowed" => blank_to_null(val, |x| builder.set_dm_upload_allowed(x)),
        "Enhances" => blank_to_null(val, |x| builder.set_enhances(x)),
        "Essential" => blank_to_null(val, |x| builder.set_essential(x)),
        "Filename" => blank_to_null(val, |x| builder.set_filename(x)),
        "Go-Import-Path" => blank_to_null(val, |x| builder.set_go_import_path(x)),
        "Homepage" => blank_to_null(val, |x| builder.set_homepage(x)),
        "Important" => blank_to_null(val, |x| builder.set_important(x)),
        "Installed-Size" => blank_to_null(val, |x| builder.set_installed_size(x)),
        "Installer-Menu-Item" => blank_to_null(val, |x| builder.set_installer_menu_item(x)),
        "Kernel-Version" => blank_to_null(val, |x| builder.set_kernel_version(x)),
        "MD5sum" => blank_to_null(val, |x| builder.set_md5sum(x)),
        "MSDOS-Filename" => blank_to_null(val, |x| builder.set_msdos_filename(x)),
        "Maintainer" => blank_to_null(val, |x| builder.set_maintainer(x)),
        "Multi-Arch" => blank_to_null(val, |x| builder.set_multi_arch(x)),
        "Optional" => blank_to_null(val, |x| builder.set_optional(x)),
        "Orig-Vcs-Browser" => blank_to_null(val, |x| builder.set_orig_vcs_browser(x)),
        "Orig-Vcs-Git" => blank_to_null(val, |x| builder.set_orig_vcs_git(x)),
        "Orig-Vcs-Svn" => blank_to_null(val, |x| builder.set_orig_vcs_svn(x)),
        "Origin" => blank_to_null(val, |x| builder.set_origin(x)),
        "Original-Maintainer" => blank_to_null(val, |x| builder.set_original_maintainer(x)),
        "Original-Vcs-Browser" => blank_to_null(val, |x| builder.set_original_vcs_browser(x)),
        "Original-Vcs-Bzr" => blank_to_null(val, |x| builder.set_original_vcs_bzr(x)),
        "Package-Revision" => blank_to_null(val, |x| builder.set_package_revision(x)),
        "Package-Type" => blank_to_null(val, |x| builder.set_package_type(x)),
        "Package_Revision" => blank_to_null(val, |x| builder.set_package_revision(x)),
        "Pre-Depends" => blank_to_null(val, |x| builder.set_pre_depends(x)),
        "Provides" => blank_to_null(val, |x| builder.set_provides(x)),
        "Python-Version" => blank_to_null(val, |x| builder.set_python_version(x)),
        "Python3-Version" => blank_to_null(val, |x| builder.set_python3_version(x)),
        "Recommended" => blank_to_null(val, |x| builder.set_recommended(x)),
        "Recommends" => blank_to_null(val, |x| builder.set_recommends(x)),
        "Replaces" => blank_to_null(val, |x| builder.set_replaces(x)),
        "Revision" => blank_to_null(val, |x| builder.set_revision(x)),
        "Ruby-Versions" => blank_to_null(val, |x| builder.set_ruby_versions(x)),
        "SHA1" => blank_to_null(val, |x| builder.set_sha1(x)),
        "SHA256" => blank_to_null(val, |x| builder.set_sha256(x)),
        "SHA512" => blank_to_null(val, |x| builder.set_sha512(x)),
        "Section" => blank_to_null(val, |x| builder.set_section(x)),
        "Size" => blank_to_null(val, |x| builder.set_size(x)),
        "Standards-Version" => blank_to_null(val, |x| builder.set_standards_version(x)),
        "Status" => blank_to_null(val, |x| builder.set_status(x)),
        "Subarchitecture" => blank_to_null(val, |x| builder.set_subarchitecture(x)),
        "Suggests" => blank_to_null(val, |x| builder.set_suggests(x)),
        "Tag" => blank_to_null(val, |x| builder.set_tag(x)),
        "Task" => blank_to_null(val, |x| builder.set_task(x)),
        "Testsuite" => blank_to_null(val, |x| builder.set_testsuite(x)),
        "Testsuite-Triggers" => blank_to_null(val, |x| builder.set_testsuite_triggers(x)),
        "Triggers-Awaited" => blank_to_null(val, |x| builder.set_triggers_awaited(x)),
        "Triggers-Pending" => blank_to_null(val, |x| builder.set_triggers_pending(x)),
        "Uploaders" => blank_to_null(val, |x| builder.set_uploaders(x)),
        "Upstream-Vcs-Bzr" => blank_to_null(val, |x| builder.set_upstream_vcs_bzr(x)),
        "Vcs-Upstream-Bzr" => blank_to_null(val, |x| builder.set_vcs_upstream_bzr(x)),

        other => bail!("unrecognised field: {}", other), 
    }

    Ok(())
}
