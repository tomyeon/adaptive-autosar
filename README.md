# adaptive-autosar
adaptive autosar written by Rust

Goal
 - tooless rust based adaptive autosar
 - pure rust adaptive platform and application

Platform Configuration Structures

    /usr/bin/oara  (RO_OARA_ROOT)
            |- EM
            |- SM
            ...
            \- Others
    /etc/oara  (ORRA_CONFIG)
            |- machine_manifest.yaml
            |- exec
                |- em_execution_manifest.yaml
                |- sm_execution_manifest.yaml
                ...
                \- others_execution_manifest.yaml
    /opt/oara (RW_OARA_ROOT)
            |- App1
            |   |- bin - App1
            |   \- manifest - app1_em_manifest.yaml
            |- others
            ...

How to use EM
    Execution management

    Usage: em.exe [OPTIONS]

    Options:
        --ro-oara-root <RO_OARA_ROOT>  read-only root path [default: /usr/bin/oara]
        --rw-oara-root <RW_OARA_ROOT>  r/w root path [default: /opt/oara]
    -c, --config <CONFIG>              configuration path [default: /etc/oara]
    -h, --help                         Print help
    -V, --version                      Print versio
