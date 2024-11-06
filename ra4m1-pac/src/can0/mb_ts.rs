#[doc = "Register `MB%s_TS` reader"]
pub type R = crate::R<MB_TS_SPEC>;
#[doc = "Register `MB%s_TS` writer"]
pub type W = crate::W<MB_TS_SPEC>;
#[doc = "Field `TSL` reader - Time Stamp Higher Byte Bits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSL_R = crate::FieldReader;
#[doc = "Field `TSL` writer - Time Stamp Higher Byte Bits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TSH` reader - Time Stamp Lower Byte Bits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSH_R = crate::FieldReader;
#[doc = "Field `TSH` writer - Time Stamp Lower Byte Bits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
pub type TSH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Higher Byte Bits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsl(&self) -> TSL_R {
        TSL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Lower Byte Bits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsh(&self) -> TSH_R {
        TSH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Higher Byte Bits TSL\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn tsl(&mut self) -> TSL_W<MB_TS_SPEC, 0> {
        TSL_W::new(self)
    }
    #[doc = "Bits 8:15 - Time Stamp Lower Byte Bits TSH\\[7:0\\]
store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn tsh(&mut self) -> TSH_W<MB_TS_SPEC, 8> {
        TSH_W::new(self)
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
#[doc = "Mailbox Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mb_ts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mb_ts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MB_TS_SPEC;
impl crate::RegisterSpec for MB_TS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mb_ts::R`](R) reader structure"]
impl crate::Readable for MB_TS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mb_ts::W`](W) writer structure"]
impl crate::Writable for MB_TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MB%s_TS to value 0"]
impl crate::Resettable for MB_TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
