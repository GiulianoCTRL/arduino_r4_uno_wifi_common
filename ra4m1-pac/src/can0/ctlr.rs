#[doc = "Register `CTLR` reader"]
pub type R = crate::R<CTLR_SPEC>;
#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CTLR_SPEC>;
#[doc = "Field `MBM` reader - CAN Mailbox Mode Select"]
pub type MBM_R = crate::BitReader<MBM_A>;
#[doc = "CAN Mailbox Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBM_A {
    #[doc = "0: Normal mailbox mode"]
    _0 = 0,
    #[doc = "1: FIFO mailbox mode"]
    _1 = 1,
}
impl From<MBM_A> for bool {
    #[inline(always)]
    fn from(variant: MBM_A) -> Self {
        variant as u8 != 0
    }
}
impl MBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBM_A {
        match self.bits {
            false => MBM_A::_0,
            true => MBM_A::_1,
        }
    }
    #[doc = "Normal mailbox mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MBM_A::_0
    }
    #[doc = "FIFO mailbox mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MBM_A::_1
    }
}
#[doc = "Field `MBM` writer - CAN Mailbox Mode Select"]
pub type MBM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MBM_A>;
impl<'a, REG, const O: u8> MBM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mailbox mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MBM_A::_0)
    }
    #[doc = "FIFO mailbox mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MBM_A::_1)
    }
}
#[doc = "Field `IDFM` reader - ID Format Mode Select"]
pub type IDFM_R = crate::FieldReader<IDFM_A>;
#[doc = "ID Format Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDFM_A {
    #[doc = "0: Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    _00 = 0,
    #[doc = "1: Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    _01 = 1,
    #[doc = "2: Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\]
to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\]
is used for the transmit FIFO."]
    _10 = 2,
    #[doc = "3: Do not use this combination"]
    _11 = 3,
}
impl From<IDFM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDFM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDFM_A {
    type Ux = u8;
}
impl IDFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDFM_A {
        match self.bits {
            0 => IDFM_A::_00,
            1 => IDFM_A::_01,
            2 => IDFM_A::_10,
            3 => IDFM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDFM_A::_00
    }
    #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDFM_A::_01
    }
    #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\]
to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\]
is used for the transmit FIFO."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDFM_A::_10
    }
    #[doc = "Do not use this combination"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDFM_A::_11
    }
}
#[doc = "Field `IDFM` writer - ID Format Mode Select"]
pub type IDFM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, IDFM_A>;
impl<'a, REG, const O: u8> IDFM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_00)
    }
    #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_01)
    }
    #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\]
to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\]
is used for the transmit FIFO."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_10)
    }
    #[doc = "Do not use this combination"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(IDFM_A::_11)
    }
}
#[doc = "Field `MLM` reader - Message Lost Mode Select"]
pub type MLM_R = crate::BitReader<MLM_A>;
#[doc = "Message Lost Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MLM_A {
    #[doc = "0: Overwrite mode"]
    _0 = 0,
    #[doc = "1: Overrun mode"]
    _1 = 1,
}
impl From<MLM_A> for bool {
    #[inline(always)]
    fn from(variant: MLM_A) -> Self {
        variant as u8 != 0
    }
}
impl MLM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MLM_A {
        match self.bits {
            false => MLM_A::_0,
            true => MLM_A::_1,
        }
    }
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MLM_A::_0
    }
    #[doc = "Overrun mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MLM_A::_1
    }
}
#[doc = "Field `MLM` writer - Message Lost Mode Select"]
pub type MLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MLM_A>;
impl<'a, REG, const O: u8> MLM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MLM_A::_0)
    }
    #[doc = "Overrun mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MLM_A::_1)
    }
}
#[doc = "Field `TPM` reader - Transmission Priority Mode Select"]
pub type TPM_R = crate::BitReader<TPM_A>;
#[doc = "Transmission Priority Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPM_A {
    #[doc = "0: ID priority transmit mode"]
    _0 = 0,
    #[doc = "1: Mailbox number priority transmit mode"]
    _1 = 1,
}
impl From<TPM_A> for bool {
    #[inline(always)]
    fn from(variant: TPM_A) -> Self {
        variant as u8 != 0
    }
}
impl TPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPM_A {
        match self.bits {
            false => TPM_A::_0,
            true => TPM_A::_1,
        }
    }
    #[doc = "ID priority transmit mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPM_A::_0
    }
    #[doc = "Mailbox number priority transmit mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPM_A::_1
    }
}
#[doc = "Field `TPM` writer - Transmission Priority Mode Select"]
pub type TPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TPM_A>;
impl<'a, REG, const O: u8> TPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ID priority transmit mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TPM_A::_0)
    }
    #[doc = "Mailbox number priority transmit mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TPM_A::_1)
    }
}
#[doc = "Field `TSRC` reader - Time Stamp Counter Reset Command"]
pub type TSRC_R = crate::BitReader<TSRC_A>;
#[doc = "Time Stamp Counter Reset Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSRC_A {
    #[doc = "0: Nothing occurred"]
    _0 = 0,
    #[doc = "1: Reset"]
    _1 = 1,
}
impl From<TSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl TSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSRC_A {
        match self.bits {
            false => TSRC_A::_0,
            true => TSRC_A::_1,
        }
    }
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSRC_A::_0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSRC_A::_1
    }
}
#[doc = "Field `TSRC` writer - Time Stamp Counter Reset Command"]
pub type TSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TSRC_A>;
impl<'a, REG, const O: u8> TSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TSRC_A::_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TSRC_A::_1)
    }
}
#[doc = "Field `TSPS` reader - Time Stamp Prescaler Select"]
pub type TSPS_R = crate::FieldReader<TSPS_A>;
#[doc = "Time Stamp Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSPS_A {
    #[doc = "0: Every bit time"]
    _00 = 0,
    #[doc = "1: Every 2-bit time"]
    _01 = 1,
    #[doc = "2: Every 4-bit time"]
    _10 = 2,
    #[doc = "3: Every 8-bit time"]
    _11 = 3,
}
impl From<TSPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSPS_A {
    type Ux = u8;
}
impl TSPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSPS_A {
        match self.bits {
            0 => TSPS_A::_00,
            1 => TSPS_A::_01,
            2 => TSPS_A::_10,
            3 => TSPS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Every bit time"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TSPS_A::_00
    }
    #[doc = "Every 2-bit time"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TSPS_A::_01
    }
    #[doc = "Every 4-bit time"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TSPS_A::_10
    }
    #[doc = "Every 8-bit time"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TSPS_A::_11
    }
}
#[doc = "Field `TSPS` writer - Time Stamp Prescaler Select"]
pub type TSPS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TSPS_A>;
impl<'a, REG, const O: u8> TSPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Every bit time"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_00)
    }
    #[doc = "Every 2-bit time"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_01)
    }
    #[doc = "Every 4-bit time"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_10)
    }
    #[doc = "Every 8-bit time"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TSPS_A::_11)
    }
}
#[doc = "Field `CANM` reader - CAN Operating Mode Select"]
pub type CANM_R = crate::FieldReader<CANM_A>;
#[doc = "CAN Operating Mode Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CANM_A {
    #[doc = "0: CAN operation mode"]
    _00 = 0,
    #[doc = "1: CAN reset mode"]
    _01 = 1,
    #[doc = "2: CAN halt mode"]
    _10 = 2,
    #[doc = "3: CAN reset mode (forcible transition)"]
    _11 = 3,
}
impl From<CANM_A> for u8 {
    #[inline(always)]
    fn from(variant: CANM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CANM_A {
    type Ux = u8;
}
impl CANM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CANM_A {
        match self.bits {
            0 => CANM_A::_00,
            1 => CANM_A::_01,
            2 => CANM_A::_10,
            3 => CANM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "CAN operation mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CANM_A::_00
    }
    #[doc = "CAN reset mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CANM_A::_01
    }
    #[doc = "CAN halt mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CANM_A::_10
    }
    #[doc = "CAN reset mode (forcible transition)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CANM_A::_11
    }
}
#[doc = "Field `CANM` writer - CAN Operating Mode Select"]
pub type CANM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CANM_A>;
impl<'a, REG, const O: u8> CANM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CAN operation mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_00)
    }
    #[doc = "CAN reset mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_01)
    }
    #[doc = "CAN halt mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_10)
    }
    #[doc = "CAN reset mode (forcible transition)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(CANM_A::_11)
    }
}
#[doc = "Field `SLPM` reader - CAN Sleep Mode"]
pub type SLPM_R = crate::BitReader<SLPM_A>;
#[doc = "CAN Sleep Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLPM_A {
    #[doc = "0: Other than CAN sleep mode"]
    _0 = 0,
    #[doc = "1: CAN sleep mode"]
    _1 = 1,
}
impl From<SLPM_A> for bool {
    #[inline(always)]
    fn from(variant: SLPM_A) -> Self {
        variant as u8 != 0
    }
}
impl SLPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLPM_A {
        match self.bits {
            false => SLPM_A::_0,
            true => SLPM_A::_1,
        }
    }
    #[doc = "Other than CAN sleep mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLPM_A::_0
    }
    #[doc = "CAN sleep mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLPM_A::_1
    }
}
#[doc = "Field `SLPM` writer - CAN Sleep Mode"]
pub type SLPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SLPM_A>;
impl<'a, REG, const O: u8> SLPM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Other than CAN sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLPM_A::_0)
    }
    #[doc = "CAN sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLPM_A::_1)
    }
}
#[doc = "Field `BOM` reader - Bus-Off Recovery Mode by a program request"]
pub type BOM_R = crate::FieldReader<BOM_A>;
#[doc = "Bus-Off Recovery Mode by a program request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOM_A {
    #[doc = "0: Normal mode (ISO11898-1 compliant)"]
    _00 = 0,
    #[doc = "1: Entry to CAN halt mode automatically at bus-off entry"]
    _01 = 1,
    #[doc = "2: Entry to CAN halt mode automatically at bus-off end"]
    _10 = 2,
    #[doc = "3: Entry to CAN halt mode (during bus-off recovery period)"]
    _11 = 3,
}
impl From<BOM_A> for u8 {
    #[inline(always)]
    fn from(variant: BOM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOM_A {
    type Ux = u8;
}
impl BOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOM_A {
        match self.bits {
            0 => BOM_A::_00,
            1 => BOM_A::_01,
            2 => BOM_A::_10,
            3 => BOM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode (ISO11898-1 compliant)"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BOM_A::_00
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BOM_A::_01
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off end"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BOM_A::_10
    }
    #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BOM_A::_11
    }
}
#[doc = "Field `BOM` writer - Bus-Off Recovery Mode by a program request"]
pub type BOM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BOM_A>;
impl<'a, REG, const O: u8> BOM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode (ISO11898-1 compliant)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_00)
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_01)
    }
    #[doc = "Entry to CAN halt mode automatically at bus-off end"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_10)
    }
    #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(BOM_A::_11)
    }
}
#[doc = "Field `RBOC` reader - Forcible Return From Bus-Off"]
pub type RBOC_R = crate::BitReader<RBOC_A>;
#[doc = "Forcible Return From Bus-Off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBOC_A {
    #[doc = "0: Nothing occurred"]
    _0 = 0,
    #[doc = "1: Forcible return from bus-off"]
    _1 = 1,
}
impl From<RBOC_A> for bool {
    #[inline(always)]
    fn from(variant: RBOC_A) -> Self {
        variant as u8 != 0
    }
}
impl RBOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBOC_A {
        match self.bits {
            false => RBOC_A::_0,
            true => RBOC_A::_1,
        }
    }
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBOC_A::_0
    }
    #[doc = "Forcible return from bus-off"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBOC_A::_1
    }
}
#[doc = "Field `RBOC` writer - Forcible Return From Bus-Off"]
pub type RBOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RBOC_A>;
impl<'a, REG, const O: u8> RBOC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Nothing occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RBOC_A::_0)
    }
    #[doc = "Forcible return from bus-off"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RBOC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Mailbox Mode Select"]
    #[inline(always)]
    pub fn mbm(&self) -> MBM_R {
        MBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ID Format Mode Select"]
    #[inline(always)]
    pub fn idfm(&self) -> IDFM_R {
        IDFM_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Message Lost Mode Select"]
    #[inline(always)]
    pub fn mlm(&self) -> MLM_R {
        MLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Priority Mode Select"]
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time Stamp Counter Reset Command"]
    #[inline(always)]
    pub fn tsrc(&self) -> TSRC_R {
        TSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Time Stamp Prescaler Select"]
    #[inline(always)]
    pub fn tsps(&self) -> TSPS_R {
        TSPS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CAN Operating Mode Select"]
    #[inline(always)]
    pub fn canm(&self) -> CANM_R {
        CANM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - CAN Sleep Mode"]
    #[inline(always)]
    pub fn slpm(&self) -> SLPM_R {
        SLPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Bus-Off Recovery Mode by a program request"]
    #[inline(always)]
    pub fn bom(&self) -> BOM_R {
        BOM_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Forcible Return From Bus-Off"]
    #[inline(always)]
    pub fn rboc(&self) -> RBOC_R {
        RBOC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Mailbox Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mbm(&mut self) -> MBM_W<CTLR_SPEC, 0> {
        MBM_W::new(self)
    }
    #[doc = "Bits 1:2 - ID Format Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn idfm(&mut self) -> IDFM_W<CTLR_SPEC, 1> {
        IDFM_W::new(self)
    }
    #[doc = "Bit 3 - Message Lost Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mlm(&mut self) -> MLM_W<CTLR_SPEC, 3> {
        MLM_W::new(self)
    }
    #[doc = "Bit 4 - Transmission Priority Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpm(&mut self) -> TPM_W<CTLR_SPEC, 4> {
        TPM_W::new(self)
    }
    #[doc = "Bit 5 - Time Stamp Counter Reset Command"]
    #[inline(always)]
    #[must_use]
    pub fn tsrc(&mut self) -> TSRC_W<CTLR_SPEC, 5> {
        TSRC_W::new(self)
    }
    #[doc = "Bits 6:7 - Time Stamp Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn tsps(&mut self) -> TSPS_W<CTLR_SPEC, 6> {
        TSPS_W::new(self)
    }
    #[doc = "Bits 8:9 - CAN Operating Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn canm(&mut self) -> CANM_W<CTLR_SPEC, 8> {
        CANM_W::new(self)
    }
    #[doc = "Bit 10 - CAN Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn slpm(&mut self) -> SLPM_W<CTLR_SPEC, 10> {
        SLPM_W::new(self)
    }
    #[doc = "Bits 11:12 - Bus-Off Recovery Mode by a program request"]
    #[inline(always)]
    #[must_use]
    pub fn bom(&mut self) -> BOM_W<CTLR_SPEC, 11> {
        BOM_W::new(self)
    }
    #[doc = "Bit 13 - Forcible Return From Bus-Off"]
    #[inline(always)]
    #[must_use]
    pub fn rboc(&mut self) -> RBOC_W<CTLR_SPEC, 13> {
        RBOC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctlr::R`](R) reader structure"]
impl crate::Readable for CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR to value 0x0500"]
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0500;
}
