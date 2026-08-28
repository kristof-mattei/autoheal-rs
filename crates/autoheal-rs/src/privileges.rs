use rustix::io::Errno;
use rustix::thread::{
    CapabilitySet, CapabilitySets, capabilities, capability_is_in_bounding_set,
    remove_capability_from_bounding_set, set_capabilities, set_no_new_privs,
};

/// Root is only needed to open the docker socket, which requires no capabilities.
pub fn drop_all() -> Result<(), Errno> {
    // changing the bounding set needs `CAP_SETPCAP`, which we don't have when running as non-root
    if capabilities(None)?
        .effective
        .contains(CapabilitySet::SETPCAP)
    {
        drop_bounding_set()?;
    }

    set_capabilities(
        None,
        CapabilitySets {
            effective: CapabilitySet::empty(),
            permitted: CapabilitySet::empty(),
            inheritable: CapabilitySet::empty(),
        },
    )?;

    set_no_new_privs(true)
}

fn drop_bounding_set() -> Result<(), Errno> {
    for bit in 0..u64::BITS {
        let capability = CapabilitySet::from_bits_retain(1_u64 << bit);

        match capability_is_in_bounding_set(capability) {
            Ok(true) => {},
            Ok(false) => continue,
            // no more capabilities to probe
            Err(Errno::INVAL) => break,
            Err(error) => return Err(error),
        }

        remove_capability_from_bounding_set(capability)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rustix::thread::{
        CapabilitySet, capabilities, capability_is_in_bounding_set, no_new_privs,
    };

    use super::drop_all;

    #[test]
    fn sets_no_new_privs() {
        drop_all().expect("Dropping privileges should work");

        assert!(
            no_new_privs().expect("Reading no_new_privs should work"),
            "no_new_privs is not set"
        );
    }

    #[test]
    #[ignore = "needs `CAP_SETPCAP`, run as root"]
    fn clears_all_sets_as_root() {
        assert!(
            capabilities(None)
                .expect("Reading capabilities should work")
                .effective
                .contains(CapabilitySet::SETPCAP),
            "Not running with CAP_SETPCAP"
        );

        drop_all().expect("Dropping privileges should work");

        let sets = capabilities(None).expect("Reading capabilities should work");

        assert!(sets.effective.is_empty(), "Effective set is not empty");
        assert!(sets.permitted.is_empty(), "Permitted set is not empty");
        assert!(sets.inheritable.is_empty(), "Inheritable set is not empty");

        for (name, capability) in CapabilitySet::all().iter_names() {
            assert!(
                !capability_is_in_bounding_set(capability)
                    .expect("Reading bounding set should work"),
                "{} is still in the bounding set",
                name
            );
        }

        assert!(
            no_new_privs().expect("Reading no_new_privs should work"),
            "no_new_privs is not set"
        );
    }
}
