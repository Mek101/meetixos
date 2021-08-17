/*! x86_64 Interrupt descriptor table */

use core::mem::size_of;

use bits::bit_fields::TBitFields;

use crate::{
    addr::virt_addr::VirtAddr,
    arch::x86_64::desc_table::{
        CpuRingMode,
        DescTablePtr
    }
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
     * Constructs an uninitialized `IntrDescTable`
     */
    pub fn new_uninitialized() -> Self {
        Self { m_intr_service_routines: [Entry { m_ptr_low: 0,
                                                m_gdt_selector: 0,
                                                m_options: 0,
                                                m_ptr_middle: 0,
                                                m_ptr_high: 0,
                                                m_reserved: 0 };
                                         256] }
    }
}

impl IntrDescTable /* Methods */ {
    /**
     * Initializes this `IntrDescTable` and loads it into the CPU
     */
    pub fn init_and_flush(&mut self) {
        {
            self.m_intr_service_routines[0] = Self::make_entry(isr_0);
            self.m_intr_service_routines[1] = Self::make_entry(isr_1);
            self.m_intr_service_routines[2] = Self::make_entry(isr_2);
            self.m_intr_service_routines[3] = Self::make_entry(isr_3);
            self.m_intr_service_routines[4] = Self::make_entry(isr_4);
            self.m_intr_service_routines[5] = Self::make_entry(isr_5);
            self.m_intr_service_routines[6] = Self::make_entry(isr_6);
            self.m_intr_service_routines[7] = Self::make_entry(isr_7);
            self.m_intr_service_routines[8] = Self::make_entry(isr_8);
            self.m_intr_service_routines[9] = Self::make_entry(isr_9);
            self.m_intr_service_routines[10] = Self::make_entry(isr_10);
            self.m_intr_service_routines[11] = Self::make_entry(isr_11);
            self.m_intr_service_routines[12] = Self::make_entry(isr_12);
            self.m_intr_service_routines[13] = Self::make_entry(isr_13);
            self.m_intr_service_routines[14] = Self::make_entry(isr_14);
            self.m_intr_service_routines[15] = Self::make_entry(isr_15);
            self.m_intr_service_routines[16] = Self::make_entry(isr_16);
            self.m_intr_service_routines[17] = Self::make_entry(isr_17);
            self.m_intr_service_routines[18] = Self::make_entry(isr_18);
            self.m_intr_service_routines[19] = Self::make_entry(isr_19);
            self.m_intr_service_routines[20] = Self::make_entry(isr_20);
            self.m_intr_service_routines[21] = Self::make_entry(isr_21);
            self.m_intr_service_routines[22] = Self::make_entry(isr_22);
            self.m_intr_service_routines[23] = Self::make_entry(isr_23);
            self.m_intr_service_routines[24] = Self::make_entry(isr_24);
            self.m_intr_service_routines[25] = Self::make_entry(isr_25);
            self.m_intr_service_routines[26] = Self::make_entry(isr_26);
            self.m_intr_service_routines[27] = Self::make_entry(isr_27);
            self.m_intr_service_routines[28] = Self::make_entry(isr_28);
            self.m_intr_service_routines[29] = Self::make_entry(isr_29);
            self.m_intr_service_routines[30] = Self::make_entry(isr_30);
            self.m_intr_service_routines[31] = Self::make_entry(isr_31);
            self.m_intr_service_routines[32] = Self::make_entry(isr_32);
            self.m_intr_service_routines[33] = Self::make_entry(isr_33);
            self.m_intr_service_routines[34] = Self::make_entry(isr_34);
            self.m_intr_service_routines[35] = Self::make_entry(isr_35);
            self.m_intr_service_routines[36] = Self::make_entry(isr_36);
            self.m_intr_service_routines[37] = Self::make_entry(isr_37);
            self.m_intr_service_routines[38] = Self::make_entry(isr_38);
            self.m_intr_service_routines[39] = Self::make_entry(isr_39);
            self.m_intr_service_routines[40] = Self::make_entry(isr_40);
            self.m_intr_service_routines[41] = Self::make_entry(isr_41);
            self.m_intr_service_routines[42] = Self::make_entry(isr_42);
            self.m_intr_service_routines[43] = Self::make_entry(isr_43);
            self.m_intr_service_routines[44] = Self::make_entry(isr_44);
            self.m_intr_service_routines[45] = Self::make_entry(isr_45);
            self.m_intr_service_routines[46] = Self::make_entry(isr_46);
            self.m_intr_service_routines[47] = Self::make_entry(isr_47);
            self.m_intr_service_routines[48] = Self::make_entry(isr_48);
            self.m_intr_service_routines[49] = Self::make_entry(isr_49);
            self.m_intr_service_routines[50] = Self::make_entry(isr_50);
            self.m_intr_service_routines[51] = Self::make_entry(isr_51);
            self.m_intr_service_routines[52] = Self::make_entry(isr_52);
            self.m_intr_service_routines[53] = Self::make_entry(isr_53);
            self.m_intr_service_routines[54] = Self::make_entry(isr_54);
            self.m_intr_service_routines[55] = Self::make_entry(isr_55);
            self.m_intr_service_routines[56] = Self::make_entry(isr_56);
            self.m_intr_service_routines[57] = Self::make_entry(isr_57);
            self.m_intr_service_routines[58] = Self::make_entry(isr_58);
            self.m_intr_service_routines[59] = Self::make_entry(isr_59);
            self.m_intr_service_routines[60] = Self::make_entry(isr_60);
            self.m_intr_service_routines[61] = Self::make_entry(isr_61);
            self.m_intr_service_routines[62] = Self::make_entry(isr_62);
            self.m_intr_service_routines[63] = Self::make_entry(isr_63);
            self.m_intr_service_routines[64] = Self::make_entry(isr_64);
            self.m_intr_service_routines[65] = Self::make_entry(isr_65);
            self.m_intr_service_routines[66] = Self::make_entry(isr_66);
            self.m_intr_service_routines[67] = Self::make_entry(isr_67);
            self.m_intr_service_routines[68] = Self::make_entry(isr_68);
            self.m_intr_service_routines[69] = Self::make_entry(isr_69);
            self.m_intr_service_routines[70] = Self::make_entry(isr_70);
            self.m_intr_service_routines[71] = Self::make_entry(isr_71);
            self.m_intr_service_routines[72] = Self::make_entry(isr_72);
            self.m_intr_service_routines[73] = Self::make_entry(isr_73);
            self.m_intr_service_routines[74] = Self::make_entry(isr_74);
            self.m_intr_service_routines[75] = Self::make_entry(isr_75);
            self.m_intr_service_routines[76] = Self::make_entry(isr_76);
            self.m_intr_service_routines[77] = Self::make_entry(isr_77);
            self.m_intr_service_routines[78] = Self::make_entry(isr_78);
            self.m_intr_service_routines[79] = Self::make_entry(isr_79);
            self.m_intr_service_routines[80] = Self::make_entry(isr_80);
            self.m_intr_service_routines[81] = Self::make_entry(isr_81);
            self.m_intr_service_routines[82] = Self::make_entry(isr_82);
            self.m_intr_service_routines[83] = Self::make_entry(isr_83);
            self.m_intr_service_routines[84] = Self::make_entry(isr_84);
            self.m_intr_service_routines[85] = Self::make_entry(isr_85);
            self.m_intr_service_routines[86] = Self::make_entry(isr_86);
            self.m_intr_service_routines[87] = Self::make_entry(isr_87);
            self.m_intr_service_routines[88] = Self::make_entry(isr_88);
            self.m_intr_service_routines[89] = Self::make_entry(isr_89);
            self.m_intr_service_routines[90] = Self::make_entry(isr_90);
            self.m_intr_service_routines[91] = Self::make_entry(isr_91);
            self.m_intr_service_routines[92] = Self::make_entry(isr_92);
            self.m_intr_service_routines[93] = Self::make_entry(isr_93);
            self.m_intr_service_routines[94] = Self::make_entry(isr_94);
            self.m_intr_service_routines[95] = Self::make_entry(isr_95);
            self.m_intr_service_routines[96] = Self::make_entry(isr_96);
            self.m_intr_service_routines[97] = Self::make_entry(isr_97);
            self.m_intr_service_routines[98] = Self::make_entry(isr_98);
            self.m_intr_service_routines[99] = Self::make_entry(isr_99);
            self.m_intr_service_routines[100] = Self::make_entry(isr_100);
            self.m_intr_service_routines[101] = Self::make_entry(isr_101);
            self.m_intr_service_routines[102] = Self::make_entry(isr_102);
            self.m_intr_service_routines[103] = Self::make_entry(isr_103);
            self.m_intr_service_routines[104] = Self::make_entry(isr_104);
            self.m_intr_service_routines[105] = Self::make_entry(isr_105);
            self.m_intr_service_routines[106] = Self::make_entry(isr_106);
            self.m_intr_service_routines[107] = Self::make_entry(isr_107);
            self.m_intr_service_routines[108] = Self::make_entry(isr_108);
            self.m_intr_service_routines[109] = Self::make_entry(isr_109);
            self.m_intr_service_routines[110] = Self::make_entry(isr_110);
            self.m_intr_service_routines[111] = Self::make_entry(isr_111);
            self.m_intr_service_routines[112] = Self::make_entry(isr_112);
            self.m_intr_service_routines[113] = Self::make_entry(isr_113);
            self.m_intr_service_routines[114] = Self::make_entry(isr_114);
            self.m_intr_service_routines[115] = Self::make_entry(isr_115);
            self.m_intr_service_routines[116] = Self::make_entry(isr_116);
            self.m_intr_service_routines[117] = Self::make_entry(isr_117);
            self.m_intr_service_routines[118] = Self::make_entry(isr_118);
            self.m_intr_service_routines[119] = Self::make_entry(isr_119);
            self.m_intr_service_routines[120] = Self::make_entry(isr_120);
            self.m_intr_service_routines[121] = Self::make_entry(isr_121);
            self.m_intr_service_routines[122] = Self::make_entry(isr_122);
            self.m_intr_service_routines[123] = Self::make_entry(isr_123);
            self.m_intr_service_routines[124] = Self::make_entry(isr_124);
            self.m_intr_service_routines[125] = Self::make_entry(isr_125);
            self.m_intr_service_routines[126] = Self::make_entry(isr_126);
            self.m_intr_service_routines[127] = Self::make_entry(isr_127);
            self.m_intr_service_routines[128] = Self::make_entry(isr_128);
            self.m_intr_service_routines[129] = Self::make_entry(isr_129);
            self.m_intr_service_routines[130] = Self::make_entry(isr_130);
            self.m_intr_service_routines[131] = Self::make_entry(isr_131);
            self.m_intr_service_routines[132] = Self::make_entry(isr_132);
            self.m_intr_service_routines[133] = Self::make_entry(isr_133);
            self.m_intr_service_routines[134] = Self::make_entry(isr_134);
            self.m_intr_service_routines[135] = Self::make_entry(isr_135);
            self.m_intr_service_routines[136] = Self::make_entry(isr_136);
            self.m_intr_service_routines[137] = Self::make_entry(isr_137);
            self.m_intr_service_routines[138] = Self::make_entry(isr_138);
            self.m_intr_service_routines[139] = Self::make_entry(isr_139);
            self.m_intr_service_routines[140] = Self::make_entry(isr_140);
            self.m_intr_service_routines[141] = Self::make_entry(isr_141);
            self.m_intr_service_routines[142] = Self::make_entry(isr_142);
            self.m_intr_service_routines[143] = Self::make_entry(isr_143);
            self.m_intr_service_routines[144] = Self::make_entry(isr_144);
            self.m_intr_service_routines[145] = Self::make_entry(isr_145);
            self.m_intr_service_routines[146] = Self::make_entry(isr_146);
            self.m_intr_service_routines[147] = Self::make_entry(isr_147);
            self.m_intr_service_routines[148] = Self::make_entry(isr_148);
            self.m_intr_service_routines[149] = Self::make_entry(isr_149);
            self.m_intr_service_routines[150] = Self::make_entry(isr_150);
            self.m_intr_service_routines[151] = Self::make_entry(isr_151);
            self.m_intr_service_routines[152] = Self::make_entry(isr_152);
            self.m_intr_service_routines[153] = Self::make_entry(isr_153);
            self.m_intr_service_routines[154] = Self::make_entry(isr_154);
            self.m_intr_service_routines[155] = Self::make_entry(isr_155);
            self.m_intr_service_routines[156] = Self::make_entry(isr_156);
            self.m_intr_service_routines[157] = Self::make_entry(isr_157);
            self.m_intr_service_routines[158] = Self::make_entry(isr_158);
            self.m_intr_service_routines[159] = Self::make_entry(isr_159);
            self.m_intr_service_routines[160] = Self::make_entry(isr_160);
            self.m_intr_service_routines[161] = Self::make_entry(isr_161);
            self.m_intr_service_routines[162] = Self::make_entry(isr_162);
            self.m_intr_service_routines[163] = Self::make_entry(isr_163);
            self.m_intr_service_routines[164] = Self::make_entry(isr_164);
            self.m_intr_service_routines[165] = Self::make_entry(isr_165);
            self.m_intr_service_routines[166] = Self::make_entry(isr_166);
            self.m_intr_service_routines[167] = Self::make_entry(isr_167);
            self.m_intr_service_routines[168] = Self::make_entry(isr_168);
            self.m_intr_service_routines[169] = Self::make_entry(isr_169);
            self.m_intr_service_routines[170] = Self::make_entry(isr_170);
            self.m_intr_service_routines[171] = Self::make_entry(isr_171);
            self.m_intr_service_routines[172] = Self::make_entry(isr_172);
            self.m_intr_service_routines[173] = Self::make_entry(isr_173);
            self.m_intr_service_routines[174] = Self::make_entry(isr_174);
            self.m_intr_service_routines[175] = Self::make_entry(isr_175);
            self.m_intr_service_routines[176] = Self::make_entry(isr_176);
            self.m_intr_service_routines[177] = Self::make_entry(isr_177);
            self.m_intr_service_routines[178] = Self::make_entry(isr_178);
            self.m_intr_service_routines[179] = Self::make_entry(isr_179);
            self.m_intr_service_routines[180] = Self::make_entry(isr_180);
            self.m_intr_service_routines[181] = Self::make_entry(isr_181);
            self.m_intr_service_routines[182] = Self::make_entry(isr_182);
            self.m_intr_service_routines[183] = Self::make_entry(isr_183);
            self.m_intr_service_routines[184] = Self::make_entry(isr_184);
            self.m_intr_service_routines[185] = Self::make_entry(isr_185);
            self.m_intr_service_routines[186] = Self::make_entry(isr_186);
            self.m_intr_service_routines[187] = Self::make_entry(isr_187);
            self.m_intr_service_routines[188] = Self::make_entry(isr_188);
            self.m_intr_service_routines[189] = Self::make_entry(isr_189);
            self.m_intr_service_routines[190] = Self::make_entry(isr_190);
            self.m_intr_service_routines[191] = Self::make_entry(isr_191);
            self.m_intr_service_routines[192] = Self::make_entry(isr_192);
            self.m_intr_service_routines[193] = Self::make_entry(isr_193);
            self.m_intr_service_routines[194] = Self::make_entry(isr_194);
            self.m_intr_service_routines[195] = Self::make_entry(isr_195);
            self.m_intr_service_routines[196] = Self::make_entry(isr_196);
            self.m_intr_service_routines[197] = Self::make_entry(isr_197);
            self.m_intr_service_routines[198] = Self::make_entry(isr_198);
            self.m_intr_service_routines[199] = Self::make_entry(isr_199);
            self.m_intr_service_routines[200] = Self::make_entry(isr_200);
            self.m_intr_service_routines[201] = Self::make_entry(isr_201);
            self.m_intr_service_routines[202] = Self::make_entry(isr_202);
            self.m_intr_service_routines[203] = Self::make_entry(isr_203);
            self.m_intr_service_routines[204] = Self::make_entry(isr_204);
            self.m_intr_service_routines[205] = Self::make_entry(isr_205);
            self.m_intr_service_routines[206] = Self::make_entry(isr_206);
            self.m_intr_service_routines[207] = Self::make_entry(isr_207);
            self.m_intr_service_routines[208] = Self::make_entry(isr_208);
            self.m_intr_service_routines[209] = Self::make_entry(isr_209);
            self.m_intr_service_routines[210] = Self::make_entry(isr_210);
            self.m_intr_service_routines[211] = Self::make_entry(isr_211);
            self.m_intr_service_routines[212] = Self::make_entry(isr_212);
            self.m_intr_service_routines[213] = Self::make_entry(isr_213);
            self.m_intr_service_routines[214] = Self::make_entry(isr_214);
            self.m_intr_service_routines[215] = Self::make_entry(isr_215);
            self.m_intr_service_routines[216] = Self::make_entry(isr_216);
            self.m_intr_service_routines[217] = Self::make_entry(isr_217);
            self.m_intr_service_routines[218] = Self::make_entry(isr_218);
            self.m_intr_service_routines[219] = Self::make_entry(isr_219);
            self.m_intr_service_routines[220] = Self::make_entry(isr_220);
            self.m_intr_service_routines[221] = Self::make_entry(isr_221);
            self.m_intr_service_routines[222] = Self::make_entry(isr_222);
            self.m_intr_service_routines[223] = Self::make_entry(isr_223);
            self.m_intr_service_routines[224] = Self::make_entry(isr_224);
            self.m_intr_service_routines[225] = Self::make_entry(isr_225);
            self.m_intr_service_routines[226] = Self::make_entry(isr_226);
            self.m_intr_service_routines[227] = Self::make_entry(isr_227);
            self.m_intr_service_routines[228] = Self::make_entry(isr_228);
            self.m_intr_service_routines[229] = Self::make_entry(isr_229);
            self.m_intr_service_routines[230] = Self::make_entry(isr_230);
            self.m_intr_service_routines[231] = Self::make_entry(isr_231);
            self.m_intr_service_routines[232] = Self::make_entry(isr_232);
            self.m_intr_service_routines[233] = Self::make_entry(isr_233);
            self.m_intr_service_routines[234] = Self::make_entry(isr_234);
            self.m_intr_service_routines[235] = Self::make_entry(isr_235);
            self.m_intr_service_routines[236] = Self::make_entry(isr_236);
            self.m_intr_service_routines[237] = Self::make_entry(isr_237);
            self.m_intr_service_routines[238] = Self::make_entry(isr_238);
            self.m_intr_service_routines[239] = Self::make_entry(isr_239);
            self.m_intr_service_routines[240] = Self::make_entry(isr_240);
            self.m_intr_service_routines[241] = Self::make_entry(isr_241);
            self.m_intr_service_routines[242] = Self::make_entry(isr_242);
            self.m_intr_service_routines[243] = Self::make_entry(isr_243);
            self.m_intr_service_routines[244] = Self::make_entry(isr_244);
            self.m_intr_service_routines[245] = Self::make_entry(isr_245);
            self.m_intr_service_routines[246] = Self::make_entry(isr_246);
            self.m_intr_service_routines[247] = Self::make_entry(isr_247);
            self.m_intr_service_routines[248] = Self::make_entry(isr_248);
            self.m_intr_service_routines[249] = Self::make_entry(isr_249);
            self.m_intr_service_routines[250] = Self::make_entry(isr_250);
            self.m_intr_service_routines[251] = Self::make_entry(isr_251);
            self.m_intr_service_routines[252] = Self::make_entry(isr_252);
            self.m_intr_service_routines[253] = Self::make_entry(isr_253);
            self.m_intr_service_routines[254] = Self::make_entry(isr_254);
            self.m_intr_service_routines[255] = Self::make_entry(isr_255);
        }

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
        let isr_fn_virt_addr = {
            /* convert first to VirtAddr for security */
            let isr_fn_virt_addr: VirtAddr = (isr_fn as *const IsrFn as usize).into();

            *isr_fn_virt_addr
        };

        /* read the current GDT selector for code-segment */
        let code_segment_selector: u16;
        unsafe {
            asm!("mov {0:x}, cs",
                 out(reg) code_segment_selector,
                 options(nomem, nostack, preserves_flags));
        }

        /* set the entry options */
        let mut entry_options = 0;
        entry_options.set_bits(13..15, CpuRingMode::Ring0 as u16);
        entry_options.set_bit(15, true); /* present bit */

        Entry { m_ptr_low: isr_fn_virt_addr as u16,
                m_gdt_selector: code_segment_selector,
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
#[derive(Copy, Clone)]
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
