/*! x86_64 Interrupt descriptor table */

use core::mem::size_of;

use bits::bit_fields::TBitFields;

use crate::arch::x86_64::{
    desc_table::{
        CpuRingMode,
        DescTablePtr
    },
    global_desc_table::SegmentSelector
};

/**
 * Convenient type renaming for ISR functions
 */
type IsrFn = unsafe extern "C" fn();

/**
 * x86_64 per CPU interrupt descriptor table
 */
#[repr(C)]
#[repr(align(16))]
pub struct IntrDescTable {
    m_intr_service_routines: [Entry; 256]
}

impl IntrDescTable /* Constructors */ {
    /**
     * Constructs a filled `IntrDescTable`
     */
    pub fn new() -> Self {
        Self { m_intr_service_routines: [Self::make_entry(isr_0),
                                         Self::make_entry(isr_1),
                                         Self::make_entry(isr_2),
                                         Self::make_entry(isr_3),
                                         Self::make_entry(isr_4),
                                         Self::make_entry(isr_5),
                                         Self::make_entry(isr_6),
                                         Self::make_entry(isr_7),
                                         Self::make_entry(isr_8),
                                         Self::make_entry(isr_9),
                                         Self::make_entry(isr_10),
                                         Self::make_entry(isr_11),
                                         Self::make_entry(isr_12),
                                         Self::make_entry(isr_13),
                                         Self::make_entry(isr_14),
                                         Self::make_entry(isr_15),
                                         Self::make_entry(isr_16),
                                         Self::make_entry(isr_17),
                                         Self::make_entry(isr_18),
                                         Self::make_entry(isr_19),
                                         Self::make_entry(isr_20),
                                         Self::make_entry(isr_21),
                                         Self::make_entry(isr_22),
                                         Self::make_entry(isr_23),
                                         Self::make_entry(isr_24),
                                         Self::make_entry(isr_25),
                                         Self::make_entry(isr_26),
                                         Self::make_entry(isr_27),
                                         Self::make_entry(isr_28),
                                         Self::make_entry(isr_29),
                                         Self::make_entry(isr_30),
                                         Self::make_entry(isr_31),
                                         Self::make_entry(isr_32),
                                         Self::make_entry(isr_33),
                                         Self::make_entry(isr_34),
                                         Self::make_entry(isr_35),
                                         Self::make_entry(isr_36),
                                         Self::make_entry(isr_37),
                                         Self::make_entry(isr_38),
                                         Self::make_entry(isr_39),
                                         Self::make_entry(isr_40),
                                         Self::make_entry(isr_41),
                                         Self::make_entry(isr_42),
                                         Self::make_entry(isr_43),
                                         Self::make_entry(isr_44),
                                         Self::make_entry(isr_45),
                                         Self::make_entry(isr_46),
                                         Self::make_entry(isr_47),
                                         Self::make_entry(isr_48),
                                         Self::make_entry(isr_49),
                                         Self::make_entry(isr_50),
                                         Self::make_entry(isr_51),
                                         Self::make_entry(isr_52),
                                         Self::make_entry(isr_53),
                                         Self::make_entry(isr_54),
                                         Self::make_entry(isr_55),
                                         Self::make_entry(isr_56),
                                         Self::make_entry(isr_57),
                                         Self::make_entry(isr_58),
                                         Self::make_entry(isr_59),
                                         Self::make_entry(isr_60),
                                         Self::make_entry(isr_61),
                                         Self::make_entry(isr_62),
                                         Self::make_entry(isr_63),
                                         Self::make_entry(isr_64),
                                         Self::make_entry(isr_65),
                                         Self::make_entry(isr_66),
                                         Self::make_entry(isr_67),
                                         Self::make_entry(isr_68),
                                         Self::make_entry(isr_69),
                                         Self::make_entry(isr_70),
                                         Self::make_entry(isr_71),
                                         Self::make_entry(isr_72),
                                         Self::make_entry(isr_73),
                                         Self::make_entry(isr_74),
                                         Self::make_entry(isr_75),
                                         Self::make_entry(isr_76),
                                         Self::make_entry(isr_77),
                                         Self::make_entry(isr_78),
                                         Self::make_entry(isr_79),
                                         Self::make_entry(isr_80),
                                         Self::make_entry(isr_81),
                                         Self::make_entry(isr_82),
                                         Self::make_entry(isr_83),
                                         Self::make_entry(isr_84),
                                         Self::make_entry(isr_85),
                                         Self::make_entry(isr_86),
                                         Self::make_entry(isr_87),
                                         Self::make_entry(isr_88),
                                         Self::make_entry(isr_89),
                                         Self::make_entry(isr_90),
                                         Self::make_entry(isr_91),
                                         Self::make_entry(isr_92),
                                         Self::make_entry(isr_93),
                                         Self::make_entry(isr_94),
                                         Self::make_entry(isr_95),
                                         Self::make_entry(isr_96),
                                         Self::make_entry(isr_97),
                                         Self::make_entry(isr_98),
                                         Self::make_entry(isr_99),
                                         Self::make_entry(isr_100),
                                         Self::make_entry(isr_101),
                                         Self::make_entry(isr_102),
                                         Self::make_entry(isr_103),
                                         Self::make_entry(isr_104),
                                         Self::make_entry(isr_105),
                                         Self::make_entry(isr_106),
                                         Self::make_entry(isr_107),
                                         Self::make_entry(isr_108),
                                         Self::make_entry(isr_109),
                                         Self::make_entry(isr_110),
                                         Self::make_entry(isr_111),
                                         Self::make_entry(isr_112),
                                         Self::make_entry(isr_113),
                                         Self::make_entry(isr_114),
                                         Self::make_entry(isr_115),
                                         Self::make_entry(isr_116),
                                         Self::make_entry(isr_117),
                                         Self::make_entry(isr_118),
                                         Self::make_entry(isr_119),
                                         Self::make_entry(isr_120),
                                         Self::make_entry(isr_121),
                                         Self::make_entry(isr_122),
                                         Self::make_entry(isr_123),
                                         Self::make_entry(isr_124),
                                         Self::make_entry(isr_125),
                                         Self::make_entry(isr_126),
                                         Self::make_entry(isr_127),
                                         Self::make_entry(isr_128),
                                         Self::make_entry(isr_129),
                                         Self::make_entry(isr_130),
                                         Self::make_entry(isr_131),
                                         Self::make_entry(isr_132),
                                         Self::make_entry(isr_133),
                                         Self::make_entry(isr_134),
                                         Self::make_entry(isr_135),
                                         Self::make_entry(isr_136),
                                         Self::make_entry(isr_137),
                                         Self::make_entry(isr_138),
                                         Self::make_entry(isr_139),
                                         Self::make_entry(isr_140),
                                         Self::make_entry(isr_141),
                                         Self::make_entry(isr_142),
                                         Self::make_entry(isr_143),
                                         Self::make_entry(isr_144),
                                         Self::make_entry(isr_145),
                                         Self::make_entry(isr_146),
                                         Self::make_entry(isr_147),
                                         Self::make_entry(isr_148),
                                         Self::make_entry(isr_149),
                                         Self::make_entry(isr_150),
                                         Self::make_entry(isr_151),
                                         Self::make_entry(isr_152),
                                         Self::make_entry(isr_153),
                                         Self::make_entry(isr_154),
                                         Self::make_entry(isr_155),
                                         Self::make_entry(isr_156),
                                         Self::make_entry(isr_157),
                                         Self::make_entry(isr_158),
                                         Self::make_entry(isr_159),
                                         Self::make_entry(isr_160),
                                         Self::make_entry(isr_161),
                                         Self::make_entry(isr_162),
                                         Self::make_entry(isr_163),
                                         Self::make_entry(isr_164),
                                         Self::make_entry(isr_165),
                                         Self::make_entry(isr_166),
                                         Self::make_entry(isr_167),
                                         Self::make_entry(isr_168),
                                         Self::make_entry(isr_169),
                                         Self::make_entry(isr_170),
                                         Self::make_entry(isr_171),
                                         Self::make_entry(isr_172),
                                         Self::make_entry(isr_173),
                                         Self::make_entry(isr_174),
                                         Self::make_entry(isr_175),
                                         Self::make_entry(isr_176),
                                         Self::make_entry(isr_177),
                                         Self::make_entry(isr_178),
                                         Self::make_entry(isr_179),
                                         Self::make_entry(isr_180),
                                         Self::make_entry(isr_181),
                                         Self::make_entry(isr_182),
                                         Self::make_entry(isr_183),
                                         Self::make_entry(isr_184),
                                         Self::make_entry(isr_185),
                                         Self::make_entry(isr_186),
                                         Self::make_entry(isr_187),
                                         Self::make_entry(isr_188),
                                         Self::make_entry(isr_189),
                                         Self::make_entry(isr_190),
                                         Self::make_entry(isr_191),
                                         Self::make_entry(isr_192),
                                         Self::make_entry(isr_193),
                                         Self::make_entry(isr_194),
                                         Self::make_entry(isr_195),
                                         Self::make_entry(isr_196),
                                         Self::make_entry(isr_197),
                                         Self::make_entry(isr_198),
                                         Self::make_entry(isr_199),
                                         Self::make_entry(isr_200),
                                         Self::make_entry(isr_201),
                                         Self::make_entry(isr_202),
                                         Self::make_entry(isr_203),
                                         Self::make_entry(isr_204),
                                         Self::make_entry(isr_205),
                                         Self::make_entry(isr_206),
                                         Self::make_entry(isr_207),
                                         Self::make_entry(isr_208),
                                         Self::make_entry(isr_209),
                                         Self::make_entry(isr_210),
                                         Self::make_entry(isr_211),
                                         Self::make_entry(isr_212),
                                         Self::make_entry(isr_213),
                                         Self::make_entry(isr_214),
                                         Self::make_entry(isr_215),
                                         Self::make_entry(isr_216),
                                         Self::make_entry(isr_217),
                                         Self::make_entry(isr_218),
                                         Self::make_entry(isr_219),
                                         Self::make_entry(isr_220),
                                         Self::make_entry(isr_221),
                                         Self::make_entry(isr_222),
                                         Self::make_entry(isr_223),
                                         Self::make_entry(isr_224),
                                         Self::make_entry(isr_225),
                                         Self::make_entry(isr_226),
                                         Self::make_entry(isr_227),
                                         Self::make_entry(isr_228),
                                         Self::make_entry(isr_229),
                                         Self::make_entry(isr_230),
                                         Self::make_entry(isr_231),
                                         Self::make_entry(isr_232),
                                         Self::make_entry(isr_233),
                                         Self::make_entry(isr_234),
                                         Self::make_entry(isr_235),
                                         Self::make_entry(isr_236),
                                         Self::make_entry(isr_237),
                                         Self::make_entry(isr_238),
                                         Self::make_entry(isr_239),
                                         Self::make_entry(isr_240),
                                         Self::make_entry(isr_241),
                                         Self::make_entry(isr_242),
                                         Self::make_entry(isr_243),
                                         Self::make_entry(isr_244),
                                         Self::make_entry(isr_245),
                                         Self::make_entry(isr_246),
                                         Self::make_entry(isr_247),
                                         Self::make_entry(isr_248),
                                         Self::make_entry(isr_249),
                                         Self::make_entry(isr_250),
                                         Self::make_entry(isr_251),
                                         Self::make_entry(isr_252),
                                         Self::make_entry(isr_253),
                                         Self::make_entry(isr_254),
                                         Self::make_entry(isr_255)] }
    }
}

impl IntrDescTable /* Methods */ {
    /**
     * Loads this `IntrDescTable` into the CPU
     */
    pub fn flush(&self) {
        /* flush this IDT */
        let intr_desc_table_ptr =
            DescTablePtr::new((size_of::<Self>() - 1) as u16,
                              (self as *const IntrDescTable).into());

        /* load the IDT pointer */
        unsafe {
            asm!("lidt [{}]",
                 in(reg) &intr_desc_table_ptr,
                 options(readonly, nostack, preserves_flags));
        }
    }
}

impl IntrDescTable /* Privates */ {
    /**
     * Constructs the `Entry` instance for the given `isr_fn`
     */
    fn make_entry(isr_fn: IsrFn) -> Entry {
        let isr_fn_virt_addr = isr_fn as *const IsrFn as usize;

        /* current GDT selector for code-segment */
        let kern_code_selector: SegmentSelector =
            SegmentSelector::C_INDEX_KERN_CODE.into();

        /* set the entry options */
        let mut entry_options = 0b1110_0000_0000;
        entry_options.set_bits(13..15, CpuRingMode::Ring0 as u16);
        entry_options.set_bit(15, true); /* present bit */

        Entry { m_ptr_low: isr_fn_virt_addr as u16,
                m_gdt_selector: kern_code_selector.as_raw() as u16,
                m_options: entry_options,
                m_ptr_middle: (isr_fn_virt_addr >> 16) as u16,
                m_ptr_high: (isr_fn_virt_addr >> 32) as u32,
                m_reserved: 0 }
    }
}

/**
 * IDT entry descriptor
 */
#[repr(C)]
#[repr(packed)]
struct Entry {
    m_ptr_low: u16,
    m_gdt_selector: u16,
    m_options: u16,
    m_ptr_middle: u16,
    m_ptr_high: u32,
    m_reserved: u32
}

/* NOTE: defined into Kernel/src/arch/x86_64/intr_service_routines.S */
extern "C" {
    fn isr_0();
    fn isr_1();
    fn isr_2();
    fn isr_3();
    fn isr_4();
    fn isr_5();
    fn isr_6();
    fn isr_7();
    fn isr_8();
    fn isr_9();
    fn isr_10();
    fn isr_11();
    fn isr_12();
    fn isr_13();
    fn isr_14();
    fn isr_15();
    fn isr_16();
    fn isr_17();
    fn isr_18();
    fn isr_19();
    fn isr_20();
    fn isr_21();
    fn isr_22();
    fn isr_23();
    fn isr_24();
    fn isr_25();
    fn isr_26();
    fn isr_27();
    fn isr_28();
    fn isr_29();
    fn isr_30();
    fn isr_31();
    fn isr_32();
    fn isr_33();
    fn isr_34();
    fn isr_35();
    fn isr_36();
    fn isr_37();
    fn isr_38();
    fn isr_39();
    fn isr_40();
    fn isr_41();
    fn isr_42();
    fn isr_43();
    fn isr_44();
    fn isr_45();
    fn isr_46();
    fn isr_47();
    fn isr_48();
    fn isr_49();
    fn isr_50();
    fn isr_51();
    fn isr_52();
    fn isr_53();
    fn isr_54();
    fn isr_55();
    fn isr_56();
    fn isr_57();
    fn isr_58();
    fn isr_59();
    fn isr_60();
    fn isr_61();
    fn isr_62();
    fn isr_63();
    fn isr_64();
    fn isr_65();
    fn isr_66();
    fn isr_67();
    fn isr_68();
    fn isr_69();
    fn isr_70();
    fn isr_71();
    fn isr_72();
    fn isr_73();
    fn isr_74();
    fn isr_75();
    fn isr_76();
    fn isr_77();
    fn isr_78();
    fn isr_79();
    fn isr_80();
    fn isr_81();
    fn isr_82();
    fn isr_83();
    fn isr_84();
    fn isr_85();
    fn isr_86();
    fn isr_87();
    fn isr_88();
    fn isr_89();
    fn isr_90();
    fn isr_91();
    fn isr_92();
    fn isr_93();
    fn isr_94();
    fn isr_95();
    fn isr_96();
    fn isr_97();
    fn isr_98();
    fn isr_99();
    fn isr_100();
    fn isr_101();
    fn isr_102();
    fn isr_103();
    fn isr_104();
    fn isr_105();
    fn isr_106();
    fn isr_107();
    fn isr_108();
    fn isr_109();
    fn isr_110();
    fn isr_111();
    fn isr_112();
    fn isr_113();
    fn isr_114();
    fn isr_115();
    fn isr_116();
    fn isr_117();
    fn isr_118();
    fn isr_119();
    fn isr_120();
    fn isr_121();
    fn isr_122();
    fn isr_123();
    fn isr_124();
    fn isr_125();
    fn isr_126();
    fn isr_127();
    fn isr_128();
    fn isr_129();
    fn isr_130();
    fn isr_131();
    fn isr_132();
    fn isr_133();
    fn isr_134();
    fn isr_135();
    fn isr_136();
    fn isr_137();
    fn isr_138();
    fn isr_139();
    fn isr_140();
    fn isr_141();
    fn isr_142();
    fn isr_143();
    fn isr_144();
    fn isr_145();
    fn isr_146();
    fn isr_147();
    fn isr_148();
    fn isr_149();
    fn isr_150();
    fn isr_151();
    fn isr_152();
    fn isr_153();
    fn isr_154();
    fn isr_155();
    fn isr_156();
    fn isr_157();
    fn isr_158();
    fn isr_159();
    fn isr_160();
    fn isr_161();
    fn isr_162();
    fn isr_163();
    fn isr_164();
    fn isr_165();
    fn isr_166();
    fn isr_167();
    fn isr_168();
    fn isr_169();
    fn isr_170();
    fn isr_171();
    fn isr_172();
    fn isr_173();
    fn isr_174();
    fn isr_175();
    fn isr_176();
    fn isr_177();
    fn isr_178();
    fn isr_179();
    fn isr_180();
    fn isr_181();
    fn isr_182();
    fn isr_183();
    fn isr_184();
    fn isr_185();
    fn isr_186();
    fn isr_187();
    fn isr_188();
    fn isr_189();
    fn isr_190();
    fn isr_191();
    fn isr_192();
    fn isr_193();
    fn isr_194();
    fn isr_195();
    fn isr_196();
    fn isr_197();
    fn isr_198();
    fn isr_199();
    fn isr_200();
    fn isr_201();
    fn isr_202();
    fn isr_203();
    fn isr_204();
    fn isr_205();
    fn isr_206();
    fn isr_207();
    fn isr_208();
    fn isr_209();
    fn isr_210();
    fn isr_211();
    fn isr_212();
    fn isr_213();
    fn isr_214();
    fn isr_215();
    fn isr_216();
    fn isr_217();
    fn isr_218();
    fn isr_219();
    fn isr_220();
    fn isr_221();
    fn isr_222();
    fn isr_223();
    fn isr_224();
    fn isr_225();
    fn isr_226();
    fn isr_227();
    fn isr_228();
    fn isr_229();
    fn isr_230();
    fn isr_231();
    fn isr_232();
    fn isr_233();
    fn isr_234();
    fn isr_235();
    fn isr_236();
    fn isr_237();
    fn isr_238();
    fn isr_239();
    fn isr_240();
    fn isr_241();
    fn isr_242();
    fn isr_243();
    fn isr_244();
    fn isr_245();
    fn isr_246();
    fn isr_247();
    fn isr_248();
    fn isr_249();
    fn isr_250();
    fn isr_251();
    fn isr_252();
    fn isr_253();
    fn isr_254();
    fn isr_255();
}
