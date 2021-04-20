#[macro_export]
macro_rules! bsp_entry {
    (fn $bsp_pre_init:path | fn $bsp_init:path) => {
        bootloader::entry_point! {__arch_bsp_entry}

        pub fn __arch_bsp_entry(bootloader_infos: &'static bootloader::BootInfo) -> ! {
            use $crate::boot::infos::BootInfos;

            /* ensure that the given function path respects the expected prototype */
            let bsp_pre_init: fn() = $bsp_pre_init;
            let bsp_init: fn() -> ! = $bsp_init;

            /* initialize for the first time the global bootloader informations, after
             * this point they can be freely obtainable via BootInfos::new() without
             * computational costs
             */
            let _infos = BootInfos::from(bootloader_infos as *const bootloader::BootInfo
                                         as *const u8);

            /* initialize the HAL for the tasks that is possible do without physical
             * and dynamic memory
             */
            $crate::boot::bsp_pre_init_hal();

            /* give to the kernel code to possibility to initialize the first stuffs,
             * like the physical and the dynamic memory management
             */
            bsp_pre_init();

            /* initialize the other stuffs of the HAL that requires physical/dynamic
             * memory
             */
            $crate::boot::bsp_post_init_hal();

            /* okay now the HAL is completely initialized, so now leave the control to
             * the kernel's common code; the HAL will be present as background to
             * support the common code
             */
            bsp_init();
        }
    };
}

#[macro_export]
macro_rules! ap_entry {
    ($func_path:path) => {};
}
