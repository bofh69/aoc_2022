// SPDX-FileCopyrightText: 2022 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;
// use rayon::prelude::*;

type InputType = String;
type SolutionType = i64;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input.lines().map(|s| s.to_owned()).collect()
}

// The input, transformed with two regexps.
// humn->HUMN modified for part2.
// The cargo fmt to format it:
fn tsnz() -> SolutionType {
    2
}
fn hrfv() -> SolutionType {
    smns() - nfng()
}
fn wfhj() -> SolutionType {
    2
}
fn rlqf() -> SolutionType {
    3
}
fn pngc() -> SolutionType {
    vdvn() + qtgs()
}
fn hmtl() -> SolutionType {
    3
}
fn jdtd() -> SolutionType {
    8
}
fn vgvd() -> SolutionType {
    2
}
fn gmnw() -> SolutionType {
    tlgd() - ddzn()
}
fn bnpd() -> SolutionType {
    gwfw() + gscj()
}
fn fpjq() -> SolutionType {
    3
}
fn vdlz() -> SolutionType {
    ldbn() + snzt()
}
fn vbbn() -> SolutionType {
    8
}
fn qhln() -> SolutionType {
    14
}
fn zlsh() -> SolutionType {
    1
}
fn fbqb() -> SolutionType {
    lldv() + tmjt()
}
fn bgzv() -> SolutionType {
    2
}
fn hgsl() -> SolutionType {
    2
}
fn ffrl() -> SolutionType {
    hbqz() + hntc()
}
fn bzvc() -> SolutionType {
    vqgz() - smth()
}
fn vnbc() -> SolutionType {
    nwwm() + jgjc()
}
fn gpsg() -> SolutionType {
    fqvb() * mgjd()
}
fn gqrv() -> SolutionType {
    llwb() + dmmt()
}
fn nrdl() -> SolutionType {
    5
}
fn tnsc() -> SolutionType {
    ttvc() * mdwt()
}
fn hhfl() -> SolutionType {
    zhhl() + flmf()
}
fn rgzn() -> SolutionType {
    glpv() + sgjf()
}
fn bvsz() -> SolutionType {
    2
}
fn cfsn() -> SolutionType {
    2
}
fn pthd() -> SolutionType {
    tnmf() * zdfh()
}
fn pnfz() -> SolutionType {
    wqhs() * rhhz()
}
fn vrjs() -> SolutionType {
    4
}
fn cdsv() -> SolutionType {
    jbtg() + flbv()
}
fn gzgv() -> SolutionType {
    mgcv() + tjbj()
}
fn cntt() -> SolutionType {
    hspg() * cdjb()
}
fn cngl() -> SolutionType {
    qvct() * rwnf()
}
fn zlvt() -> SolutionType {
    dwgs() / vdrj()
}
fn zjhr() -> SolutionType {
    2
}
fn snpf() -> SolutionType {
    bsdr() * plqn()
}
fn qvwm() -> SolutionType {
    3
}
fn sjqr() -> SolutionType {
    6
}
fn dclg() -> SolutionType {
    4
}
fn pzqq() -> SolutionType {
    htbt() + fwzw()
}
fn lpll() -> SolutionType {
    15
}
fn tgvz() -> SolutionType {
    16
}
fn ldtb() -> SolutionType {
    wsnp() - tbfp()
}
fn tjcf() -> SolutionType {
    wvcr() / pjzb()
}
fn sjgr() -> SolutionType {
    3
}
fn hthb() -> SolutionType {
    3
}
fn dnhg() -> SolutionType {
    mnvs() / wcvd()
}
fn cdfp() -> SolutionType {
    jvqz() * vlmb()
}
fn djfl() -> SolutionType {
    jdvn() * pqcz()
}
fn mhvw() -> SolutionType {
    zrqd() + vvdm()
}
fn zmsf() -> SolutionType {
    jqtt() * jwbg()
}
fn hswt() -> SolutionType {
    1
}
fn thgv() -> SolutionType {
    2
}
fn vnmq() -> SolutionType {
    lgrn() + ggst()
}
fn zhmr() -> SolutionType {
    1
}
fn tvbh() -> SolutionType {
    nwmr() + wshw()
}
fn vdcv() -> SolutionType {
    mvsl() + mcsn()
}
fn nmbj() -> SolutionType {
    3
}
fn mrfp() -> SolutionType {
    2
}
fn wdwr() -> SolutionType {
    cffj() - fhgj()
}
fn bpzt() -> SolutionType {
    7
}
fn ltgr() -> SolutionType {
    2
}
fn qqhj() -> SolutionType {
    12
}
fn tljh() -> SolutionType {
    tvvh() * qwll()
}
fn bnbt() -> SolutionType {
    mgwn() * mqmn()
}
fn vppf() -> SolutionType {
    hflt() * pqmd()
}
fn dqlr() -> SolutionType {
    zzsr() * lzqb()
}
fn ctdm() -> SolutionType {
    2
}
fn vqgz() -> SolutionType {
    dzfv() / qhtn()
}
fn wlnq() -> SolutionType {
    4
}
fn lzqb() -> SolutionType {
    cblv() - tqhf()
}
fn dmgs() -> SolutionType {
    frjn() * jvsb()
}
fn rsvc() -> SolutionType {
    2
}
fn llqh() -> SolutionType {
    4
}
fn mlzq() -> SolutionType {
    12
}
fn nnvb() -> SolutionType {
    4
}
fn bwnd() -> SolutionType {
    qvld() / rdgg()
}
fn gmlr() -> SolutionType {
    dvgd() * mjgc()
}
fn lzvh() -> SolutionType {
    9
}
fn jqrj() -> SolutionType {
    5
}
fn mcdw() -> SolutionType {
    5
}
fn jhwl() -> SolutionType {
    swvn() * tvgn()
}
fn dgvr() -> SolutionType {
    ggbb() * hftm()
}
fn rvps() -> SolutionType {
    pjgq() * rcfj()
}
fn vnsz() -> SolutionType {
    2
}
fn ptdp() -> SolutionType {
    dhhj() * grcr()
}
fn bcdw() -> SolutionType {
    13
}
fn thrn() -> SolutionType {
    5
}
fn ljwm() -> SolutionType {
    5
}
fn hwff() -> SolutionType {
    8
}
fn zmgm() -> SolutionType {
    2
}
fn mqcr() -> SolutionType {
    tmvb() * vjdl()
}
fn bbll() -> SolutionType {
    gnfq() * qjgn()
}
fn dvgd() -> SolutionType {
    12
}
fn dwnn() -> SolutionType {
    dprq() * qllt()
}
fn gbtm() -> SolutionType {
    3
}
fn stcg() -> SolutionType {
    7
}
fn hfjr() -> SolutionType {
    jwqz() * tpdj()
}
fn jzph() -> SolutionType {
    2
}
fn gjvw() -> SolutionType {
    9
}
fn tzqc() -> SolutionType {
    vmvt() + plns()
}
fn lpvd() -> SolutionType {
    6
}
fn ncwt() -> SolutionType {
    3
}
fn fpjv() -> SolutionType {
    vvvg() + rdvm()
}
fn cppz() -> SolutionType {
    3
}
fn cvpm() -> SolutionType {
    sbjl() + lwzj()
}
fn gmjj() -> SolutionType {
    19
}
fn lvzp() -> SolutionType {
    jfbp() * vhbj()
}
fn wrjd() -> SolutionType {
    bdsq() * fpjq()
}
fn dsgb() -> SolutionType {
    wrln() * cfvz()
}
fn cpzd() -> SolutionType {
    jbgs() + tsnz()
}
fn bjhw() -> SolutionType {
    qhlz() * jlzr()
}
fn mzgv() -> SolutionType {
    vjwj() + ddzj()
}
fn njzd() -> SolutionType {
    nqwh() / lsqj()
}
fn szqn() -> SolutionType {
    scld() + gpmd()
}
fn grcf() -> SolutionType {
    6
}
fn dwhs() -> SolutionType {
    phpv() * pqrf()
}
fn rsqm() -> SolutionType {
    2
}
fn phph() -> SolutionType {
    2
}
fn gsch() -> SolutionType {
    8
}
fn prmq() -> SolutionType {
    fcdn() * vrjs()
}
fn qfpf() -> SolutionType {
    sffn() / dbqv()
}
fn pqvp() -> SolutionType {
    2
}
fn nzdw() -> SolutionType {
    dflp() * cwvd()
}
fn wvjf() -> SolutionType {
    ggzg() - rnqm()
}
fn cndj() -> SolutionType {
    pngc() * sctr()
}
fn sncp() -> SolutionType {
    ccvd() + gnpg()
}
fn fpgb() -> SolutionType {
    tnsc() + tpqn()
}
fn zrqd() -> SolutionType {
    ccvj() + ztlj()
}
fn fvvn() -> SolutionType {
    2
}
fn wlgd() -> SolutionType {
    5
}
fn nwgv() -> SolutionType {
    jjcs() * tfjv()
}
fn lwwz() -> SolutionType {
    3
}
fn tmsh() -> SolutionType {
    gqrv() + rdtl()
}
fn djcm() -> SolutionType {
    1
}
fn qqhl() -> SolutionType {
    3
}
fn rvbc() -> SolutionType {
    5
}
fn grrv() -> SolutionType {
    nlrq() + npfl()
}
fn ppsl() -> SolutionType {
    twlj() + lwrl()
}
fn zjsj() -> SolutionType {
    fjlr() + dqlr()
}
fn zpwd() -> SolutionType {
    tsdd() + tbpn()
}
fn mpln() -> SolutionType {
    mvjg() * vrgj()
}
fn fjbr() -> SolutionType {
    hrcd() * rgrb()
}
fn mzvz() -> SolutionType {
    qppg() * fdsf()
}
fn gfng() -> SolutionType {
    2
}
fn gszj() -> SolutionType {
    dfjb() + llfc()
}
fn zvpd() -> SolutionType {
    mzwb() + rvps()
}
fn dmbf() -> SolutionType {
    znzh() / jvpl()
}
fn pwlc() -> SolutionType {
    dmgs() - ltbw()
}
fn jjzd() -> SolutionType {
    lvlg() / vmdq()
}
fn dbvq() -> SolutionType {
    jjzd() - prtp()
}
fn hflt() -> SolutionType {
    2
}
fn pmff() -> SolutionType {
    2
}
fn dhmt() -> SolutionType {
    5
}
fn shbm() -> SolutionType {
    2
}
fn dhnm() -> SolutionType {
    11
}
fn lppn() -> SolutionType {
    4
}
fn zrtm() -> SolutionType {
    bzzm() * pvmw()
}
fn glnt() -> SolutionType {
    5
}
fn gwfw() -> SolutionType {
    lzzq() + bbrz()
}
fn zbqv() -> SolutionType {
    jvlw() * gvjd()
}
fn sgdh() -> SolutionType {
    jtbg() / wtdm()
}
fn prpb() -> SolutionType {
    3
}
fn tndt() -> SolutionType {
    5
}
fn qhct() -> SolutionType {
    pjsw() + mmwv()
}
fn qnzc() -> SolutionType {
    3
}
fn gngd() -> SolutionType {
    gjtv() + pwtd()
}
fn thht() -> SolutionType {
    4
}
fn bcjl() -> SolutionType {
    5
}
fn rhcc() -> SolutionType {
    2
}
fn zsjp() -> SolutionType {
    vshj() + wqdv()
}
fn tctm() -> SolutionType {
    vnbp() * rpnz()
}
fn mlpv() -> SolutionType {
    hwbs() - wmsj()
}
fn twvh() -> SolutionType {
    3
}
fn jggt() -> SolutionType {
    tcdm() * blch()
}
fn pgbf() -> SolutionType {
    djwl() * jnvb()
}
fn tbfp() -> SolutionType {
    3
}
fn gzwf() -> SolutionType {
    4
}
fn mrfl() -> SolutionType {
    bgct() + ljhn()
}
fn jqpc() -> SolutionType {
    zhfv() + zjcf()
}
fn zprb() -> SolutionType {
    hfjr() + szcz()
}
fn flmf() -> SolutionType {
    thht() * fbdc()
}
fn znjr() -> SolutionType {
    2
}
fn rwwm() -> SolutionType {
    3
}
fn fpmz() -> SolutionType {
    zrpz() + frzn()
}
fn cdmz() -> SolutionType {
    4
}
fn qvlv() -> SolutionType {
    gfwh() + ggzj()
}
fn jmlr() -> SolutionType {
    12
}
fn rfsb() -> SolutionType {
    jtsf() * dhdp()
}
fn tfjv() -> SolutionType {
    3
}
fn ndvn() -> SolutionType {
    hdgm() * tnsp()
}
fn lclw() -> SolutionType {
    bbtg() + qqdg()
}
fn tfdg() -> SolutionType {
    brln() + jwlm()
}
fn jzhs() -> SolutionType {
    3
}
fn prtw() -> SolutionType {
    3
}
fn qwlr() -> SolutionType {
    4
}
fn sczz() -> SolutionType {
    wzls() * gwmt()
}
fn smth() -> SolutionType {
    bnpd() * phdc()
}
fn gwmt() -> SolutionType {
    5
}
fn mlqw() -> SolutionType {
    hvtz() + gcmd()
}
fn fchm() -> SolutionType {
    HUMN.load(Ordering::SeqCst) - fspz()
}
fn lvdd() -> SolutionType {
    2
}
fn lmmj() -> SolutionType {
    wssz() * jzhs()
}
fn llmf() -> SolutionType {
    jvnn() * sjlh()
}
fn fnsp() -> SolutionType {
    16
}
fn wmrp() -> SolutionType {
    rsvc() * rqws()
}
fn spjr() -> SolutionType {
    5
}
fn pqmd() -> SolutionType {
    3
}
fn sgdg() -> SolutionType {
    3
}
fn cdjn() -> SolutionType {
    zlvt() - dsqj()
}
fn nvwv() -> SolutionType {
    4
}
fn bbrm() -> SolutionType {
    hzlt() * fnsp()
}
fn dncm() -> SolutionType {
    2
}
fn gvbm() -> SolutionType {
    9
}
fn bptj() -> SolutionType {
    mtzt() + fmbq()
}
fn qbqz() -> SolutionType {
    jrmb() + mqcr()
}
fn lgwd() -> SolutionType {
    sdhf() * smsc()
}
fn nhch() -> SolutionType {
    lzjb() + zqcj()
}
fn frjn() -> SolutionType {
    jdgz() / bnhz()
}
fn jpth() -> SolutionType {
    4
}
fn jdhj() -> SolutionType {
    9
}
fn chgb() -> SolutionType {
    jrvr() * zqnz()
}
fn ncqp() -> SolutionType {
    cczj() + djwg()
}
fn ngwj() -> SolutionType {
    2
}
fn tgts() -> SolutionType {
    hqsf() * llpp()
}
fn htbt() -> SolutionType {
    9
}
fn tlgd() -> SolutionType {
    nvlb() * hjnq()
}
fn phdc() -> SolutionType {
    pgwd() * qsrl()
}
fn qrfq() -> SolutionType {
    12
}
fn ddqj() -> SolutionType {
    2
}
fn nrmz() -> SolutionType {
    jzwc() * nltb()
}
fn rvch() -> SolutionType {
    nslv() + mlzq()
}
fn qgnm() -> SolutionType {
    5
}
fn rvcc() -> SolutionType {
    3
}
fn bggd() -> SolutionType {
    1
}
fn rgvf() -> SolutionType {
    3
}
fn pvmw() -> SolutionType {
    2
}
fn ggbb() -> SolutionType {
    2
}
fn rsvr() -> SolutionType {
    7
}
fn srjw() -> SolutionType {
    ptbj() + mdlj()
}
fn llfc() -> SolutionType {
    16
}
fn ncwb() -> SolutionType {
    18
}
fn zlzc() -> SolutionType {
    2
}
fn wlpf() -> SolutionType {
    2
}
fn vqbr() -> SolutionType {
    tgtm() + rlrm()
}
fn vjlc() -> SolutionType {
    dzft() - cjbr()
}
fn mvcc() -> SolutionType {
    18
}
fn fqgf() -> SolutionType {
    3
}
fn mgcw() -> SolutionType {
    2
}
fn gvqs() -> SolutionType {
    20
}
fn cqrb() -> SolutionType {
    jlzj() * ldgf()
}
fn mgjd() -> SolutionType {
    3
}
fn fsrh() -> SolutionType {
    3
}
fn fqhq() -> SolutionType {
    2
}
fn mjjv() -> SolutionType {
    9
}
fn tjwd() -> SolutionType {
    fpgb() + jggt()
}
fn vqtr() -> SolutionType {
    5
}
fn mwtq() -> SolutionType {
    2
}
fn wcvd() -> SolutionType {
    3
}
fn rzsf() -> SolutionType {
    vhrd() + dplg()
}
fn szfn() -> SolutionType {
    4
}
fn ltsz() -> SolutionType {
    2
}
fn lsqj() -> SolutionType {
    2
}
fn bfpt() -> SolutionType {
    13
}
fn btvz() -> SolutionType {
    5
}
fn mcpq() -> SolutionType {
    gfng() * sbng()
}
fn qhtn() -> SolutionType {
    2
}
fn lbhp() -> SolutionType {
    2
}
fn dbqv() -> SolutionType {
    2
}
fn swvn() -> SolutionType {
    2
}
fn phpv() -> SolutionType {
    rgzn() * bvdc()
}
fn mwjg() -> SolutionType {
    1
}
fn srmj() -> SolutionType {
    3
}
fn srch() -> SolutionType {
    nqsr() + bnqv()
}
fn nfds() -> SolutionType {
    2
}
fn hsdn() -> SolutionType {
    lzwq() + gmjg()
}
fn cjbr() -> SolutionType {
    2
}
fn rmzm() -> SolutionType {
    3
}
fn hgsb() -> SolutionType {
    bsjl() - zlsh()
}
fn sfpd() -> SolutionType {
    2
}
fn qdgf() -> SolutionType {
    hznr() * lvdd()
}
fn fmfj() -> SolutionType {
    mqww() + bmwh()
}
fn nhtm() -> SolutionType {
    pqhz() / tjlc()
}
fn fhhd() -> SolutionType {
    4
}
fn cjdg() -> SolutionType {
    2
}
fn hnbg() -> SolutionType {
    5
}
fn rrvd() -> SolutionType {
    jqmj() + hvfn()
}
fn hdbp() -> SolutionType {
    4
}
fn jcwt() -> SolutionType {
    nqrp() - msth()
}
fn lzwq() -> SolutionType {
    zbjp() + bqbr()
}
fn nfhl() -> SolutionType {
    2
}
fn flbl() -> SolutionType {
    qmwt() + btvz()
}
fn fpgp() -> SolutionType {
    3
}
fn jtqj() -> SolutionType {
    5
}
fn mdlj() -> SolutionType {
    ccfh() + gwrw()
}
fn sflj() -> SolutionType {
    nbvb() * gjmv()
}
fn lfms() -> SolutionType {
    fcvp() + rpgm()
}
fn tfln() -> SolutionType {
    tvbr() * mchw()
}
fn pstz() -> SolutionType {
    scbt() * cclm()
}
fn pvpl() -> SolutionType {
    vfjc() / qgng()
}
fn nqcl() -> SolutionType {
    pqvp() + wjdd()
}
fn jcrg() -> SolutionType {
    7
}
fn ftzt() -> SolutionType {
    lpqf() * plmd()
}
fn zqmj() -> SolutionType {
    2
}
fn ldgf() -> SolutionType {
    3
}
fn zrns() -> SolutionType {
    zjsj() / ldtb()
}
fn ztlj() -> SolutionType {
    15
}
fn ppmn() -> SolutionType {
    8
}
fn btdm() -> SolutionType {
    qsbn() * zzml()
}
fn lmmh() -> SolutionType {
    13
}
fn gcqv() -> SolutionType {
    3
}
fn dbcq() -> SolutionType {
    lhrr() * jmwz()
}
fn tcqc() -> SolutionType {
    2
}
fn drmd() -> SolutionType {
    11
}
fn mdcn() -> SolutionType {
    ppfr() + vbbp()
}
fn mwtw() -> SolutionType {
    2
}
fn hhnv() -> SolutionType {
    2
}
fn gfmw() -> SolutionType {
    2
}
fn htrv() -> SolutionType {
    qfmt() * gqzc()
}
fn pzvn() -> SolutionType {
    3
}
fn ldgc() -> SolutionType {
    lhfj() * sqtg()
}
fn wmsj() -> SolutionType {
    1
}
fn zlrf() -> SolutionType {
    ndhv() * pgbf()
}
fn cvbm() -> SolutionType {
    sdrq() * wmnf()
}
fn szcz() -> SolutionType {
    2
}
fn svht() -> SolutionType {
    nlsq() * lnzg()
}
fn hzgt() -> SolutionType {
    5
}
fn pmqf() -> SolutionType {
    4
}
fn bjsm() -> SolutionType {
    lmdl() + lghh()
}
fn wcvv() -> SolutionType {
    grft() + zcnw()
}
fn prvf() -> SolutionType {
    13
}
fn rmmb() -> SolutionType {
    5
}
fn hwlv() -> SolutionType {
    5
}
fn fnlp() -> SolutionType {
    dtjn() * zvll()
}
fn pzms() -> SolutionType {
    4
}
fn ncct() -> SolutionType {
    hqjm() + ncrp()
}
fn lpfw() -> SolutionType {
    znjg() + mglg()
}
fn plbl() -> SolutionType {
    hhhz() * wczz()
}
fn nfng() -> SolutionType {
    grgr() / rtrc()
}
fn vmvt() -> SolutionType {
    wrlq() + cntt()
}
fn jrtw() -> SolutionType {
    7
}
fn zjcf() -> SolutionType {
    fjhz() + tcdt()
}
fn ldgq() -> SolutionType {
    16
}
fn tnzz() -> SolutionType {
    2
}
fn cmpz() -> SolutionType {
    mrmj() - pdmd()
}
fn lqts() -> SolutionType {
    hqsr() / tcvf()
}
fn mgfw() -> SolutionType {
    gdgw() - ddtw()
}
fn pfzv() -> SolutionType {
    ppsl() * jpmm()
}
fn cghd() -> SolutionType {
    5
}
fn wgcs() -> SolutionType {
    3
}
fn sgnh() -> SolutionType {
    4
}
fn qbzt() -> SolutionType {
    mprm() - pgrt()
}
fn nwcj() -> SolutionType {
    3
}
fn zcpn() -> SolutionType {
    smjh() * npbh()
}
fn zbfr() -> SolutionType {
    jcwt() * fsfs()
}
fn dcfv() -> SolutionType {
    zsng() + fmfp()
}
fn zczb() -> SolutionType {
    mvlb() + zrtm()
}
fn ltsj() -> SolutionType {
    9
}
fn fhpr() -> SolutionType {
    bcsr() + chvj()
}
fn fhcp() -> SolutionType {
    17
}
fn srll() -> SolutionType {
    4
}
fn sqzv() -> SolutionType {
    5
}
fn rnbp() -> SolutionType {
    jpwc() * dgbd()
}
fn jtfc() -> SolutionType {
    4
}
fn ppsh() -> SolutionType {
    sfng() * bsjn()
}
fn nrdf() -> SolutionType {
    2
}
fn fdhh() -> SolutionType {
    3
}
fn qlpz() -> SolutionType {
    4
}
fn gvwj() -> SolutionType {
    dqgj() * vmtv()
}
fn plqn() -> SolutionType {
    19
}
fn tvdn() -> SolutionType {
    2
}
fn bnnf() -> SolutionType {
    4
}
fn bsdr() -> SolutionType {
    2
}
fn qbqh() -> SolutionType {
    5
}
fn brdh() -> SolutionType {
    zvjz() + wmzb()
}
fn hjcd() -> SolutionType {
    qfwq() + mbtr()
}
fn mrwd() -> SolutionType {
    4
}
fn nqsr() -> SolutionType {
    rllf() * wrnm()
}
fn zvrl() -> SolutionType {
    rcwl() * rcbn()
}
fn vjwj() -> SolutionType {
    clqz() / twvh()
}
fn brct() -> SolutionType {
    3
}
fn zjjw() -> SolutionType {
    sjgr() * fbzw()
}
fn gmrr() -> SolutionType {
    qsqq() * nnvb()
}
fn twnv() -> SolutionType {
    3
}
fn cvfh() -> SolutionType {
    tzjw() * vjlc()
}
fn jmbw() -> SolutionType {
    pmff() * rhqs()
}
fn bbfr() -> SolutionType {
    4
}
fn fcgn() -> SolutionType {
    sgvz() + plbl()
}
fn gnfq() -> SolutionType {
    3
}
fn hnph() -> SolutionType {
    wvfg() + vrfj()
}
fn njgc() -> SolutionType {
    2
}
fn pmwl() -> SolutionType {
    2
}
fn ztfs() -> SolutionType {
    2
}
fn cbwd() -> SolutionType {
    lppj() * gsch()
}
fn zgpt() -> SolutionType {
    jtzc() / vbbn()
}
fn vsjb() -> SolutionType {
    2
}
fn ptqm() -> SolutionType {
    3
}
fn lfzg() -> SolutionType {
    2
}
fn phft() -> SolutionType {
    grnz() * hswd()
}
fn vczf() -> SolutionType {
    2
}
fn tmjt() -> SolutionType {
    2
}
fn psbz() -> SolutionType {
    9
}
fn sgvz() -> SolutionType {
    5
}
fn trhm() -> SolutionType {
    dqcj() * szvj()
}
fn zfnq() -> SolutionType {
    cbzv() * rvch()
}
fn wjvh() -> SolutionType {
    vvml() * fdhh()
}
fn jdqd() -> SolutionType {
    tldf() + bnnj()
}
fn qwtr() -> SolutionType {
    ppmn() * sqbc()
}
fn zdfm() -> SolutionType {
    10
}
fn fmbq() -> SolutionType {
    4
}
fn bpwg() -> SolutionType {
    2
}
fn cqmb() -> SolutionType {
    1
}
fn zpwc() -> SolutionType {
    sgdh() * lmmh()
}
fn rjcw() -> SolutionType {
    5
}
fn wfnh() -> SolutionType {
    9
}
fn btqz() -> SolutionType {
    qrhp() * lqts()
}
fn rczr() -> SolutionType {
    3
}
fn llsf() -> SolutionType {
    2
}
fn wllz() -> SolutionType {
    2
}
fn hswd() -> SolutionType {
    nrdl() * tnzz()
}
fn pswf() -> SolutionType {
    5
}
fn jpwc() -> SolutionType {
    gqpp() * twbm()
}
fn fjzt() -> SolutionType {
    slqb() * jrvd()
}
fn pvww() -> SolutionType {
    5
}
fn qmnt() -> SolutionType {
    rbdl() * fnhf()
}
fn qwvg() -> SolutionType {
    zpwc() + zqdh()
}
fn gdgw() -> SolutionType {
    whgc() + btwc()
}
fn vmzb() -> SolutionType {
    2
}
fn csvj() -> SolutionType {
    19
}
fn ggqb() -> SolutionType {
    1
}
fn zscp() -> SolutionType {
    6
}
fn mrdj() -> SolutionType {
    nrhc() + jjmt()
}
fn gnpg() -> SolutionType {
    20
}
fn gjmv() -> SolutionType {
    vbdm() / lppn()
}
fn gcmd() -> SolutionType {
    sdfb() * hczv()
}
fn czwt() -> SolutionType {
    3
}
fn wfpd() -> SolutionType {
    2
}
fn lrnc() -> SolutionType {
    mhmb() * vtgw()
}
fn hmvv() -> SolutionType {
    11
}
fn qsbn() -> SolutionType {
    5
}
fn nvds() -> SolutionType {
    pzpg() + dbvq()
}
fn wbzd() -> SolutionType {
    3
}
fn sppw() -> SolutionType {
    rpcz() * gtqq()
}
fn wmzb() -> SolutionType {
    pftp() + ndlz()
}
fn wldr() -> SolutionType {
    cmpz() * nsjv()
}
fn pzlq() -> SolutionType {
    fjzt() + tjwd()
}
fn vdhd() -> SolutionType {
    phrp() * djln()
}
fn sgjs() -> SolutionType {
    vzsp() - fjbr()
}
fn lzpg() -> SolutionType {
    qwtz() * nrmz()
}
fn bbtg() -> SolutionType {
    zcwj() + wzmd()
}
fn svzm() -> SolutionType {
    14
}
fn vtbj() -> SolutionType {
    vblq() + rvgc()
}
fn hzdb() -> SolutionType {
    3
}
fn scld() -> SolutionType {
    2
}
fn mghq() -> SolutionType {
    2
}
fn hzvd() -> SolutionType {
    vqcr() + tcpw()
}
fn qtwm() -> SolutionType {
    20
}
fn zjhn() -> SolutionType {
    11
}
fn rtnc() -> SolutionType {
    9
}
fn tfsg() -> SolutionType {
    gfmw() * dcvl()
}
fn scnw() -> SolutionType {
    2
}
fn mrhl() -> SolutionType {
    cltl() + ggpz()
}
fn fcbf() -> SolutionType {
    tmsh() / qzhh()
}
fn mlfn() -> SolutionType {
    bdnf() * hmtl()
}
fn rcmz() -> SolutionType {
    4
}
fn zpqp() -> SolutionType {
    2
}
fn chls() -> SolutionType {
    2
}
fn rzns() -> SolutionType {
    2
}
fn frbh() -> SolutionType {
    mgfw() / dncm()
}
fn vsmn() -> SolutionType {
    2
}
fn plmd() -> SolutionType {
    3
}
fn bnwg() -> SolutionType {
    hnhd() * hnbg()
}
fn jdvn() -> SolutionType {
    5
}
fn whjg() -> SolutionType {
    vpdp() + cpzd()
}
fn wntf() -> SolutionType {
    bccq() + ttmp()
}
fn zbgs() -> SolutionType {
    3
}
fn nfts() -> SolutionType {
    16
}
fn tdfs() -> SolutionType {
    9
}
fn tjsl() -> SolutionType {
    2
}
fn mwpz() -> SolutionType {
    tlrn() + wddr()
}
fn tbpn() -> SolutionType {
    4
}
fn vhrd() -> SolutionType {
    prtw() * gpsg()
}
fn rgnc() -> SolutionType {
    6
}
fn grcs() -> SolutionType {
    qmwl() / vspg()
}
fn tsdd() -> SolutionType {
    jrjp() + phqn()
}
fn mbtr() -> SolutionType {
    4
}
fn whnf() -> SolutionType {
    20
}
fn hrgz() -> SolutionType {
    qnhp() + sjqr()
}
fn zhdz() -> SolutionType {
    fwss() * mlfb()
}
fn lhcc() -> SolutionType {
    5
}
fn thrw() -> SolutionType {
    hwff() * dstj()
}
fn tnwd() -> SolutionType {
    llzt() / hcrf()
}
fn qsgl() -> SolutionType {
    2
}
fn sjfm() -> SolutionType {
    9
}
fn ddzn() -> SolutionType {
    7
}
fn zdcc() -> SolutionType {
    2
}
fn prtp() -> SolutionType {
    5
}
fn rpvv() -> SolutionType {
    2
}
fn lfvv() -> SolutionType {
    11
}
fn pcjn() -> SolutionType {
    3
}
fn dzfv() -> SolutionType {
    qrvj() + zrns()
}
fn qsqq() -> SolutionType {
    4
}
fn twlb() -> SolutionType {
    19
}
fn grsp() -> SolutionType {
    lpfw() + cbwd()
}
fn qfmt() -> SolutionType {
    2
}
fn plhh() -> SolutionType {
    wqcg() * gtwn()
}
fn tnmf() -> SolutionType {
    4
}
fn fmfp() -> SolutionType {
    3
}
fn hrbj() -> SolutionType {
    6
}
fn jwlm() -> SolutionType {
    17
}
fn gmjg() -> SolutionType {
    snwr() - zsdn()
}
fn jpqg() -> SolutionType {
    2
}
fn djlr() -> SolutionType {
    5
}
fn lgpq() -> SolutionType {
    qhcd() * rvbc()
}
fn qsll() -> SolutionType {
    swjw() * mwsf()
}
fn glcq() -> SolutionType {
    lmnn() / tccp()
}
fn cqzp() -> SolutionType {
    cnvf() * vbbs()
}
fn hvfn() -> SolutionType {
    tmlp() * nrsb()
}
fn nslv() -> SolutionType {
    5
}
fn cgqr() -> SolutionType {
    3
}
fn jnff() -> SolutionType {
    2
}
fn ccvd() -> SolutionType {
    9
}
fn sgqs() -> SolutionType {
    2
}
fn vnlt() -> SolutionType {
    2
}
fn nrhc() -> SolutionType {
    zbfr() + rmqc()
}
fn hwtr() -> SolutionType {
    3
}
fn vlfc() -> SolutionType {
    7
}
fn rqwp() -> SolutionType {
    1
}
fn wjdd() -> SolutionType {
    vnsz() * sbrr()
}
fn gsps() -> SolutionType {
    wntf() + svzm()
}
fn jllb() -> SolutionType {
    2
}
fn zzsr() -> SolutionType {
    hgsl() * tsvp()
}
fn ggzg() -> SolutionType {
    jrtw() * ptss()
}
fn bccq() -> SolutionType {
    1
}
fn fngd() -> SolutionType {
    14
}
fn gtqt() -> SolutionType {
    19
}
fn hpbd() -> SolutionType {
    6
}
fn gtgj() -> SolutionType {
    2
}
fn nwhf() -> SolutionType {
    nhdw() * pmwl()
}
fn dpwb() -> SolutionType {
    2
}
fn nhrp() -> SolutionType {
    mfjw() * gbfl()
}
fn hlpm() -> SolutionType {
    vrsh() + zdhc()
}
fn wnfb() -> SolutionType {
    lfnh() + hwbq()
}
fn drtf() -> SolutionType {
    3
}
fn gtzf() -> SolutionType {
    2
}
fn tmvb() -> SolutionType {
    dznp() * nfjf()
}
fn twbw() -> SolutionType {
    3
}
fn fcdn() -> SolutionType {
    4
}
fn mrjl() -> SolutionType {
    4
}
fn zhhh() -> SolutionType {
    4
}
fn htsg() -> SolutionType {
    tpfw() / lmmj()
}
fn sffn() -> SolutionType {
    hbcj() * sncp()
}
fn dddl() -> SolutionType {
    2
}
fn mzdz() -> SolutionType {
    tvbh() + tmcr()
}
fn nbvb() -> SolutionType {
    rtnc() + wdmc()
}
fn vsdp() -> SolutionType {
    lwwz() * mgcw()
}
fn cbzl() -> SolutionType {
    gbdb() + fdrz()
}
fn pgwd() -> SolutionType {
    3
}
fn qzjs() -> SolutionType {
    3
}
fn lqcf() -> SolutionType {
    grrv() * zfbp()
}
fn dqln() -> SolutionType {
    rvjn() * wblv()
}
fn qdgn() -> SolutionType {
    pdsm() * fnlp()
}
fn qppg() -> SolutionType {
    2
}
fn ccfh() -> SolutionType {
    gmrr() - fddz()
}
fn hmzw() -> SolutionType {
    wlqg() + qnwr()
}
fn tgtm() -> SolutionType {
    fsqw() * rzns()
}
fn vvhb() -> SolutionType {
    2
}
fn jbtg() -> SolutionType {
    mvvj() + nvds()
}
fn qpqz() -> SolutionType {
    fvwz() + gqnt()
}
fn vpjl() -> SolutionType {
    6
}
fn ddts() -> SolutionType {
    fcbf() * wwrg()
}
fn jcwm() -> SolutionType {
    vldd() / qsgl()
}
fn msjf() -> SolutionType {
    vdcv() - ngtp()
}
fn pdmg() -> SolutionType {
    9
}
fn ncmd() -> SolutionType {
    3
}
fn nhfn() -> SolutionType {
    4
}
fn zsdn() -> SolutionType {
    rlmd() / jzzf()
}
fn mvdv() -> SolutionType {
    tgts() + gzdv()
}
fn gfdp() -> SolutionType {
    pmjz() / rsgc()
}
fn jgbd() -> SolutionType {
    3
}
fn fqpd() -> SolutionType {
    2
}
fn bpnc() -> SolutionType {
    2
}
fn zsrw() -> SolutionType {
    glwz() + qtjv()
}
fn zmvq() -> SolutionType {
    zqmh() * rpth()
}
fn tbls() -> SolutionType {
    2
}
fn wmhn() -> SolutionType {
    20
}
fn bbdw() -> SolutionType {
    tzsc() + mjlg()
}
fn jhmb() -> SolutionType {
    7
}
fn zgnm() -> SolutionType {
    ctbb() * bqzs()
}
fn nsdq() -> SolutionType {
    5
}
fn rhqs() -> SolutionType {
    3
}
fn gmqd() -> SolutionType {
    11
}
fn twmp() -> SolutionType {
    18
}
fn dgbj() -> SolutionType {
    1
}
fn nlcw() -> SolutionType {
    hmmj() + ssfg()
}
fn zgbn() -> SolutionType {
    4
}
fn vtvb() -> SolutionType {
    crdf() + jczt()
}
fn mvls() -> SolutionType {
    zjhr() * qznl()
}
fn bjhz() -> SolutionType {
    2
}
fn lgsh() -> SolutionType {
    2
}
fn hwrm() -> SolutionType {
    nzdw() * jfvl()
}
fn ppqg() -> SolutionType {
    cjpb() * gdrr()
}
fn cfsp() -> SolutionType {
    wrjd() * sljm()
}
fn pgvc() -> SolutionType {
    jwsg() * bdbt()
}
fn ppnn() -> SolutionType {
    4
}
fn hntc() -> SolutionType {
    gszj() * bwqc()
}
fn mglg() -> SolutionType {
    rhbl() * rrwd()
}
fn gwzg() -> SolutionType {
    vpph() * mwhb()
}
fn ndhc() -> SolutionType {
    2
}
fn qzpj() -> SolutionType {
    2
}
fn qfwq() -> SolutionType {
    3
}
fn sjqv() -> SolutionType {
    twqw() + nccc()
}
fn bcjn() -> SolutionType {
    hnnt() * pdwr()
}
fn rfnq() -> SolutionType {
    2
}
fn fvld() -> SolutionType {
    hvvc() * fqpg()
}
fn lncn() -> SolutionType {
    wscd() + vmbv()
}
fn sdww() -> SolutionType {
    rgnc() + pzrl()
}
fn vqbp() -> SolutionType {
    3
}
fn psmc() -> SolutionType {
    2
}
fn lvlg() -> SolutionType {
    nnff() + csdd()
}
fn hnrm() -> SolutionType {
    wjvh() - frbh()
}
fn cwzf() -> SolutionType {
    2
}
fn lhmg() -> SolutionType {
    16
}
fn jgjc() -> SolutionType {
    gtgs() + wgjd()
}
fn dfjb() -> SolutionType {
    dfrz() + mtts()
}
fn tgmh() -> SolutionType {
    gmlr() - fvmq()
}
fn hcrj() -> SolutionType {
    hmvv() * sbpf()
}
fn wshw() -> SolutionType {
    2
}
fn jwll() -> SolutionType {
    hssq() - zgbn()
}
fn wrln() -> SolutionType {
    2
}
fn wcws() -> SolutionType {
    2
}
fn jffs() -> SolutionType {
    tndt() * rwzq()
}
fn jdlb() -> SolutionType {
    nsbp() * rbjd()
}
fn rwzn() -> SolutionType {
    2
}
fn fwzw() -> SolutionType {
    2
}
fn mthn() -> SolutionType {
    3
}
fn ftrj() -> SolutionType {
    qsll() / vvhb()
}
fn ttfq() -> SolutionType {
    8
}
fn tnjj() -> SolutionType {
    qmfq() * dsrb()
}
fn vdnr() -> SolutionType {
    6
}
fn vsdz() -> SolutionType {
    3
}
fn zqvn() -> SolutionType {
    5
}
fn bsss() -> SolutionType {
    4
}
fn fcts() -> SolutionType {
    tcqc() * bmpt()
}
fn dzdd() -> SolutionType {
    5
}
fn gjjw() -> SolutionType {
    3
}
fn brqs() -> SolutionType {
    cgqr() + mqnz()
}
fn dglf() -> SolutionType {
    16
}
fn rpjc() -> SolutionType {
    2
}
fn nwgg() -> SolutionType {
    2
}
fn hphf() -> SolutionType {
    8
}
fn dshg() -> SolutionType {
    2
}
fn bmgg() -> SolutionType {
    jbns() * ssrr()
}
fn jrmb() -> SolutionType {
    hphf() * hcrq()
}
fn ghsg() -> SolutionType {
    3
}
fn mwgd() -> SolutionType {
    3
}
fn wdmp() -> SolutionType {
    gcgw() + sdww()
}
fn tldf() -> SolutionType {
    hcvc() + hfrb()
}
fn fjhz() -> SolutionType {
    qhdg() + lgpq()
}
fn nhzs() -> SolutionType {
    ssbn() - pfqg()
}
fn dznp() -> SolutionType {
    stpm() + mzvs()
}
fn bllq() -> SolutionType {
    2
}
fn gdvt() -> SolutionType {
    qhct() * zjqf()
}
fn wqcj() -> SolutionType {
    3
}
fn wgjd() -> SolutionType {
    spjr() * vrjj()
}
fn lvnh() -> SolutionType {
    2
}
fn bnff() -> SolutionType {
    11
}
fn gvbg() -> SolutionType {
    zwnm() + vppf()
}
fn ztnw() -> SolutionType {
    5
}
fn qzrc() -> SolutionType {
    zpnw() + wtgt()
}
fn qprg() -> SolutionType {
    2
}
fn ttvc() -> SolutionType {
    dcfv() - cfhv()
}
fn mtbg() -> SolutionType {
    8
}
fn mccf() -> SolutionType {
    2
}
fn wvfg() -> SolutionType {
    vgvd() + hbqv()
}
fn fzdr() -> SolutionType {
    tcfc() / htrv()
}
fn jnsz() -> SolutionType {
    3
}
fn njnn() -> SolutionType {
    3
}
fn wtgt() -> SolutionType {
    dtdw() * cfqn()
}
fn qwll() -> SolutionType {
    6
}
fn lnzg() -> SolutionType {
    fqgf() * hzdb()
}
fn vltd() -> SolutionType {
    zbsc() * bbfr()
}
fn zcwj() -> SolutionType {
    lztj() * vlsw()
}
fn rrzl() -> SolutionType {
    13
}
fn lbhc() -> SolutionType {
    nbzg() * gmqd()
}
fn qvgv() -> SolutionType {
    zfnq() / zcsm()
}
fn cbzv() -> SolutionType {
    2
}
fn vpph() -> SolutionType {
    2
}
fn ctjz() -> SolutionType {
    fsjv() - fztr()
}
fn rdsh() -> SolutionType {
    2
}
fn qfgs() -> SolutionType {
    4
}
fn pqsc() -> SolutionType {
    3
}
fn djwg() -> SolutionType {
    ldgc() + wptw()
}
fn ssdc() -> SolutionType {
    5
}
fn pqwg() -> SolutionType {
    wpql() * vptr()
}
fn bhqh() -> SolutionType {
    qgsh() + gtpb()
}
fn bchq() -> SolutionType {
    lpqp() * fwpp()
}
fn nrdn() -> SolutionType {
    3
}
fn tpdl() -> SolutionType {
    gcqv() * gngd()
}
fn pgrt() -> SolutionType {
    2
}
fn sfvc() -> SolutionType {
    5
}
fn fdrz() -> SolutionType {
    4
}
fn dsqj() -> SolutionType {
    hhhl() + wnfb()
}
fn ggzj() -> SolutionType {
    qfpf() + pthd()
}
fn cltl() -> SolutionType {
    11
}
fn tcfc() -> SolutionType {
    pfzv() - bmgg()
}
fn rvqd() -> SolutionType {
    1
}
fn zhll() -> SolutionType {
    hbnn() * rfsq()
}
fn ljmw() -> SolutionType {
    5
}
fn wzfq() -> SolutionType {
    twrv() + rddp()
}
fn vzsp() -> SolutionType {
    hczw() * chls()
}
fn bstt() -> SolutionType {
    5
}
fn cbgz() -> SolutionType {
    jllb() * zpzf()
}
fn szjv() -> SolutionType {
    3
}
fn qlrl() -> SolutionType {
    1
}
fn fbmd() -> SolutionType {
    2
}
fn fqsm() -> SolutionType {
    5
}
fn fglf() -> SolutionType {
    2
}
fn vbqf() -> SolutionType {
    5
}
fn sctr() -> SolutionType {
    2
}
fn wrhn() -> SolutionType {
    6
}
fn lgjv() -> SolutionType {
    ffrl() / tjsl()
}
fn cgdm() -> SolutionType {
    7
}
fn hgsh() -> SolutionType {
    sjpv() * jhmb()
}
fn cpsb() -> SolutionType {
    2
}
fn rjhr() -> SolutionType {
    cwzf() * pwgs()
}
fn ccvj() -> SolutionType {
    zsjp() * jcgs()
}
fn lpqp() -> SolutionType {
    zqvn() * bwvr()
}
fn jrvr() -> SolutionType {
    2
}
fn pvjs() -> SolutionType {
    3
}
fn gbpr() -> SolutionType {
    lttb() + vpjl()
}
fn jwqz() -> SolutionType {
    3
}
fn jlfd() -> SolutionType {
    vvnn() + sfpn()
}
fn lgrn() -> SolutionType {
    tjjh() - rmmb()
}
fn lqqs() -> SolutionType {
    lzzf() + bttm()
}
fn smns() -> SolutionType {
    smlb() + brpt()
}
fn bqlh() -> SolutionType {
    3
}
fn bzzm() -> SolutionType {
    11
}
fn pqcl() -> SolutionType {
    7
}
fn lljz() -> SolutionType {
    qqhl() * qvwm()
}
fn mvjf() -> SolutionType {
    2
}
fn zpff() -> SolutionType {
    wlgd() * pqjz()
}
fn zvll() -> SolutionType {
    qmlh() / cjdc()
}
fn rbhw() -> SolutionType {
    7
}
fn tmcr() -> SolutionType {
    qhqq() + gwzg()
}
fn jwrj() -> SolutionType {
    3
}
fn qgng() -> SolutionType {
    mwmv() * wfhj()
}
fn jqmj() -> SolutionType {
    hprb() * tfsg()
}
fn tjlc() -> SolutionType {
    2
}
fn dwvq() -> SolutionType {
    3
}
fn dtwl() -> SolutionType {
    pmqf() + sqms()
}
fn hmmj() -> SolutionType {
    wzfq() + vdnr()
}
fn wmnp() -> SolutionType {
    fbfl() * bnrl()
}
fn zrgc() -> SolutionType {
    3
}
fn jvsb() -> SolutionType {
    jjpg() + hzvf()
}
fn rcnf() -> SolutionType {
    cmcj() * ddqj()
}
fn qhqq() -> SolutionType {
    wlnn() + rvmv()
}
fn cjpb() -> SolutionType {
    2
}
fn prqf() -> SolutionType {
    14
}
fn vwzf() -> SolutionType {
    vbqf() + ldgq()
}
fn dqcj() -> SolutionType {
    gsvt() + vphq()
}
fn fvsv() -> SolutionType {
    2
}
fn ftfh() -> SolutionType {
    ncwb() * lpll()
}
fn shlq() -> SolutionType {
    6
}
fn tmlp() -> SolutionType {
    5
}
fn ffcl() -> SolutionType {
    5
}
fn fmng() -> SolutionType {
    pmpj() + grtv()
}
fn hmbd() -> SolutionType {
    rgbg() - gqrt()
}
fn qlvl() -> SolutionType {
    fbqb() + tnmv()
}
fn gjwh() -> SolutionType {
    mdcn() * fhzg()
}
fn gggc() -> SolutionType {
    4
}
fn zrsw() -> SolutionType {
    5
}
fn hnnt() -> SolutionType {
    chtc() * stfd()
}
fn zqmh() -> SolutionType {
    stcg() * mwpz()
}
fn lhvs() -> SolutionType {
    2
}
fn vjss() -> SolutionType {
    lfvv() * dqdg()
}
fn wmnf() -> SolutionType {
    lclw() + srjw()
}
fn zmtj() -> SolutionType {
    njnn() + jdsz()
}
fn nlsq() -> SolutionType {
    3
}
fn cclm() -> SolutionType {
    jcwm() + zhmr()
}
fn vtgw() -> SolutionType {
    2
}
fn bfvp() -> SolutionType {
    3
}
fn wshl() -> SolutionType {
    2
}
fn bnlq() -> SolutionType {
    5
}
fn zsnp() -> SolutionType {
    3
}
fn hhtr() -> SolutionType {
    tgds() * bzvc()
}
fn jmwz() -> SolutionType {
    19
}
fn rhbl() -> SolutionType {
    3
}
fn bjrz() -> SolutionType {
    9
}
fn vgqd() -> SolutionType {
    rbsn() - pqsc()
}
fn zsdd() -> SolutionType {
    trzs() * ggvj()
}
fn ljbs() -> SolutionType {
    2
}
fn hnrl() -> SolutionType {
    prvf() + mqll()
}
fn wqcg() -> SolutionType {
    11
}
fn sfpn() -> SolutionType {
    tcdl() * cfsn()
}
fn wzdw() -> SolutionType {
    vsdp() + ttdf()
}
fn bsjl() -> SolutionType {
    8
}
fn gscj() -> SolutionType {
    14
}
fn hjnq() -> SolutionType {
    17
}
fn sfng() -> SolutionType {
    11
}
fn tpjd() -> SolutionType {
    npjd() + qwtr()
}
fn wjwq() -> SolutionType {
    tvpb() * hjcd()
}
fn dwhf() -> SolutionType {
    hwlv() * zznc()
}
fn mqww() -> SolutionType {
    5
}
fn cscs() -> SolutionType {
    19
}
fn gnth() -> SolutionType {
    gmfd() + prfn()
}
fn qvld() -> SolutionType {
    tjrt() + fsrp()
}
fn wmpf() -> SolutionType {
    lfms() * ncbz()
}
fn mfzt() -> SolutionType {
    2
}
fn bwvr() -> SolutionType {
    5
}
fn rvzz() -> SolutionType {
    zwjt() + plhh()
}
fn cvnn() -> SolutionType {
    vltd() + qtwm()
}
fn fhfv() -> SolutionType {
    qnnn() + bslh()
}
fn lmdl() -> SolutionType {
    4
}
fn mdqz() -> SolutionType {
    16
}
fn tdbz() -> SolutionType {
    3
}
fn hrvz() -> SolutionType {
    2
}
fn fcvp() -> SolutionType {
    nwhn() + sczr()
}
fn nzmz() -> SolutionType {
    7
}
fn thwd() -> SolutionType {
    vcqv() * fhcp()
}
fn tvvh() -> SolutionType {
    14
}
fn fwfg() -> SolutionType {
    vnlt() * rczr()
}
fn cqcj() -> SolutionType {
    pwvr() + jsqd()
}
fn lrfs() -> SolutionType {
    tdhd() * nchn()
}
fn nrrs() -> SolutionType {
    gmjh() + qvlv()
}
fn wssz() -> SolutionType {
    4
}
fn mmwv() -> SolutionType {
    hcnn() * nnhl()
}
fn dfrz() -> SolutionType {
    bcjl() * gvlr()
}
fn czlz() -> SolutionType {
    vprh() + gqgz()
}
fn mfjw() -> SolutionType {
    3
}
fn wdjs() -> SolutionType {
    fvvn() * nhzs()
}
fn hljw() -> SolutionType {
    scnw() * mdlw()
}
fn bdnf() -> SolutionType {
    vdgg() + hpdd()
}
fn hvwt() -> SolutionType {
    mfmh() + vvdp()
}
fn vqfz() -> SolutionType {
    1
}
fn qmwl() -> SolutionType {
    qrjg() * qbph()
}
fn pqjz() -> SolutionType {
    5
}
fn fzmh() -> SolutionType {
    rnpg() * srll()
}
fn nccc() -> SolutionType {
    jhwl() * zqmj()
}
fn rnqm() -> SolutionType {
    4
}
fn mdwt() -> SolutionType {
    3
}
fn phvm() -> SolutionType {
    gtdp() * hcqp()
}
fn tjjh() -> SolutionType {
    qcqp() * ffbs()
}
fn hfjm() -> SolutionType {
    3
}
fn chbf() -> SolutionType {
    nhgv() + nrms()
}
fn ncbz() -> SolutionType {
    5
}
fn ddvr() -> SolutionType {
    1
}
fn mhmb() -> SolutionType {
    dwmw() * dcjm()
}
fn nsvg() -> SolutionType {
    15
}
fn ddtw() -> SolutionType {
    brdh() * llhf()
}
fn mwmv() -> SolutionType {
    3
}
fn rvmv() -> SolutionType {
    2
}
fn qqdb() -> SolutionType {
    3
}
fn clgq() -> SolutionType {
    zrsw() + dwzs()
}
fn vbdm() -> SolutionType {
    vtvb() * jdtd()
}
fn jqtt() -> SolutionType {
    cdsv() + rpcb()
}
fn zjqf() -> SolutionType {
    gtqt() + rnbp()
}
fn bbrz() -> SolutionType {
    14
}
fn mcgj() -> SolutionType {
    bjrj() + tcld()
}
fn tzgg() -> SolutionType {
    13
}
fn frzn() -> SolutionType {
    1
}
fn lqmh() -> SolutionType {
    3
}
fn gtqq() -> SolutionType {
    fhfs() - twmp()
}
fn jdsz() -> SolutionType {
    4
}
fn vspg() -> SolutionType {
    4
}
fn mbnq() -> SolutionType {
    3
}
fn wrmc() -> SolutionType {
    2
}
fn grcn() -> SolutionType {
    3
}
fn pwjt() -> SolutionType {
    1
}
fn dhdp() -> SolutionType {
    7
}
fn czjv() -> SolutionType {
    3
}
fn szsv() -> SolutionType {
    pqzt() * hcsb()
}
fn mprm() -> SolutionType {
    wdrz() + ljjn()
}
fn wblq() -> SolutionType {
    llmf() / nnhg()
}
fn lqsz() -> SolutionType {
    blrl() - jwzt()
}
fn hrjs() -> SolutionType {
    4
}
fn gsvt() -> SolutionType {
    6
}
fn tfgc() -> SolutionType {
    3
}
fn fsrf() -> SolutionType {
    2
}
fn cfrh() -> SolutionType {
    gffs() * bnlq()
}
fn ptbj() -> SolutionType {
    cdhg() + fjnh()
}
fn zbsc() -> SolutionType {
    5
}
fn llbt() -> SolutionType {
    7
}
fn qwjc() -> SolutionType {
    4
}
fn msdb() -> SolutionType {
    ctzw() * lbhp()
}
fn dcjm() -> SolutionType {
    2
}
fn jbgs() -> SolutionType {
    mfbb() * fwht()
}
fn dnjz() -> SolutionType {
    9
}
fn hcpm() -> SolutionType {
    7
}
fn dwgs() -> SolutionType {
    tgbf() + zdsl()
}
fn twrv() -> SolutionType {
    vzcp() + gvbm()
}
fn nrvn() -> SolutionType {
    rzhc() + vmrd()
}
fn gpzm() -> SolutionType {
    hlpm() * dnjz()
}
fn lzrf() -> SolutionType {
    jwll() * jnhs()
}
fn chzq() -> SolutionType {
    6
}
fn nbzg() -> SolutionType {
    lwtg() / qzpj()
}
fn cczj() -> SolutionType {
    dgjd() / qwlr()
}
fn hrcd() -> SolutionType {
    19
}
fn lmnn() -> SolutionType {
    rssw() * whjg()
}
fn vvdp() -> SolutionType {
    6
}
fn dgbd() -> SolutionType {
    2
}
fn hnhd() -> SolutionType {
    5
}
fn rltv() -> SolutionType {
    2
}
fn ngcg() -> SolutionType {
    bsmt() * qshb()
}
fn fzdn() -> SolutionType {
    nwhf() * fqpd()
}
fn rdgg() -> SolutionType {
    2
}
fn dtdw() -> SolutionType {
    2
}
fn mjgc() -> SolutionType {
    3
}
fn bdbt() -> SolutionType {
    mqjm() + hcmm()
}
fn rwgg() -> SolutionType {
    11
}
fn cdcv() -> SolutionType {
    qvbd() * jqmw()
}
fn zzsc() -> SolutionType {
    zqvv() + dglf()
}
fn lzjb() -> SolutionType {
    3
}
fn crrb() -> SolutionType {
    lbhc() + gpzm()
}
fn nqrp() -> SolutionType {
    wdmp() * btzf()
}
fn wwmt() -> SolutionType {
    ngcg() - hjln()
}
fn hcmm() -> SolutionType {
    scjr() * wdwr()
}
fn dstm() -> SolutionType {
    qhln() + drlw()
}
fn nltb() -> SolutionType {
    3
}
fn qcqp() -> SolutionType {
    wrmc() * rnhn()
}
fn gcdn() -> SolutionType {
    htcl() + hcbz()
}
fn rczp() -> SolutionType {
    mmrp() + rsvv()
}
fn fjlr() -> SolutionType {
    fchm() * njsj()
}
fn zcnw() -> SolutionType {
    9
}
fn twds() -> SolutionType {
    2
}
fn dflp() -> SolutionType {
    2
}
fn bccv() -> SolutionType {
    6
}
fn pdwr() -> SolutionType {
    2
}
fn dwzs() -> SolutionType {
    bsgl() * fgjv()
}
fn hfrg() -> SolutionType {
    4
}
fn gbfl() -> SolutionType {
    2
}
fn mtzt() -> SolutionType {
    3
}
fn qmfq() -> SolutionType {
    wjbl() * wlpf()
}
fn dhvn() -> SolutionType {
    bdvh() * ccps()
}
fn llbs() -> SolutionType {
    2
}
fn jwsg() -> SolutionType {
    6
}
fn jlwz() -> SolutionType {
    7
}
fn hzlt() -> SolutionType {
    4
}
fn qwtq() -> SolutionType {
    ppld() / twbw()
}
fn gqzc() -> SolutionType {
    3
}
fn qvbd() -> SolutionType {
    5
}
fn wgsj() -> SolutionType {
    tpjd() + pptt()
}
fn rpcb() -> SolutionType {
    gqsf() + rtsw()
}
fn jqdf() -> SolutionType {
    dbzd() / rpvv()
}
fn vdjz() -> SolutionType {
    ljwm() * njgc()
}
fn pgdm() -> SolutionType {
    mrjl() + wgcs()
}
fn npwg() -> SolutionType {
    3
}
fn zgvw() -> SolutionType {
    pvpl() - vjss()
}
fn npbh() -> SolutionType {
    pbzh() * sgmz()
}
fn fzjd() -> SolutionType {
    zrcm() + qncj()
}
fn tjrt() -> SolutionType {
    mccf() * cdjn()
}
fn jbns() -> SolutionType {
    csvj() * shbm()
}
fn mcvf() -> SolutionType {
    vwzf() + ncqs()
}
fn hpdd() -> SolutionType {
    ccwd() / hhnv()
}
fn wpql() -> SolutionType {
    psvd() + pzlq()
}
fn rsvv() -> SolutionType {
    hhfl() + mwhm()
}
fn ttrq() -> SolutionType {
    2
}
fn dbnp() -> SolutionType {
    lvzp() * thrn()
}
fn dzwz() -> SolutionType {
    3
}
fn pqcz() -> SolutionType {
    5
}
fn dprq() -> SolutionType {
    rwnw() * bftj()
}
fn ljhn() -> SolutionType {
    5
}
fn llpp() -> SolutionType {
    djlr() + jzph()
}
fn dhhj() -> SolutionType {
    10
}
fn gtwn() -> SolutionType {
    cfrh() + rvjs()
}
fn jndn() -> SolutionType {
    2
}
fn llgd() -> SolutionType {
    ltsz() * msdb()
}
fn frnp() -> SolutionType {
    tljh() + zsrw()
}
fn dbgc() -> SolutionType {
    2
}
fn smjh() -> SolutionType {
    wpbt() - szqn()
}
fn qmlh() -> SolutionType {
    ljbs() * bwch()
}
fn dphb() -> SolutionType {
    mwtq() * gqtb()
}
fn lggw() -> SolutionType {
    wdlb() + zvrl()
}
fn qznl() -> SolutionType {
    5
}
fn fnhf() -> SolutionType {
    4
}
fn vmrd() -> SolutionType {
    10
}
fn lztj() -> SolutionType {
    2
}
fn pzpg() -> SolutionType {
    8
}
fn rnpw() -> SolutionType {
    lzqq() / nlwz()
}
fn pvln() -> SolutionType {
    5
}
fn mbsc() -> SolutionType {
    wmgn() * hsgr()
}
fn lpwg() -> SolutionType {
    2
}
fn vwtd() -> SolutionType {
    svht() * dbfq()
}
fn bmpt() -> SolutionType {
    4
}
fn fpmd() -> SolutionType {
    pvjs() * sjtt()
}
fn vtlz() -> SolutionType {
    dtwl() * zmgm()
}
fn nvlb() -> SolutionType {
    2
}
fn gwrw() -> SolutionType {
    ndcf() + ljsr()
}
fn wvcr() -> SolutionType {
    ssdc() * wvbg()
}
fn rjzt() -> SolutionType {
    14
}
fn vzrv() -> SolutionType {
    2
}
fn gtpb() -> SolutionType {
    wldr() + qwvg()
}
fn zccz() -> SolutionType {
    2
}
fn hwwb() -> SolutionType {
    5
}
fn sbvw() -> SolutionType {
    7
}
fn sbrr() -> SolutionType {
    4
}
fn zntf() -> SolutionType {
    2
}
fn cdsl() -> SolutionType {
    2
}
fn tcvf() -> SolutionType {
    4
}
fn fdgq() -> SolutionType {
    dgwj() * htfd()
}
fn ltbv() -> SolutionType {
    psmc() * grcn()
}
fn mblw() -> SolutionType {
    wptb() + wshl()
}
fn cqrp() -> SolutionType {
    cnsb() + rgqr()
}
fn nlrq() -> SolutionType {
    9
}
fn ptss() -> SolutionType {
    5
}
fn zqvv() -> SolutionType {
    3
}
fn fdfj() -> SolutionType {
    bptj() * mngq()
}
fn gcgw() -> SolutionType {
    2
}
fn mjlg() -> SolutionType {
    glcq() * cjqm()
}
fn pjzb() -> SolutionType {
    5
}
fn bnrl() -> SolutionType {
    jnff() * hmzw()
}
fn mvwb() -> SolutionType {
    5
}
fn dlbj() -> SolutionType {
    srch() + vnmq()
}
fn wmgn() -> SolutionType {
    3
}
fn qjgp() -> SolutionType {
    rhcc() + pdmg()
}
fn zsrq() -> SolutionType {
    lptf() / bqzn()
}
fn vshj() -> SolutionType {
    3
}
fn zdhc() -> SolutionType {
    12
}
fn swnn() -> SolutionType {
    4
}
fn msnd() -> SolutionType {
    6
}
fn nhdw() -> SolutionType {
    zntf() * nlcw()
}
fn wzls() -> SolutionType {
    bqlz() + vtbj()
}
fn qnnn() -> SolutionType {
    vlcb() * vjgg()
}
fn nsbp() -> SolutionType {
    jcrg() + qrfq()
}
fn hwbs() -> SolutionType {
    mjqt() + hrsl()
}
fn cjqm() -> SolutionType {
    7
}
fn prdz() -> SolutionType {
    zrfj() + dsln()
}
fn pzzl() -> SolutionType {
    czlc() * nmgj()
}
fn twhf() -> SolutionType {
    6
}
fn mgwn() -> SolutionType {
    tvdn() + chgb()
}
fn ggst() -> SolutionType {
    1
}
fn dgwj() -> SolutionType {
    pvln() * ccdj()
}
fn qwbj() -> SolutionType {
    5
}
fn cwvd() -> SolutionType {
    13
}
fn mcsn() -> SolutionType {
    rjhr() + ndtn()
}
fn frbg() -> SolutionType {
    rjhc() + mdzl()
}
fn bhcz() -> SolutionType {
    2
}
fn jcsf() -> SolutionType {
    8
}
fn zwjt() -> SolutionType {
    mlfn() + hhtr()
}
fn dqgj() -> SolutionType {
    3
}
fn dhpp() -> SolutionType {
    zjhn() * nvwv()
}
fn rcwl() -> SolutionType {
    2
}
fn mqjm() -> SolutionType {
    4
}
fn pcnn() -> SolutionType {
    9
}
fn vfch() -> SolutionType {
    2
}
fn nnhl() -> SolutionType {
    vdjz() + tdfs()
}
fn sprb() -> SolutionType {
    zjjw() + djdj()
}
fn nwhd() -> SolutionType {
    4
}
fn jjcs() -> SolutionType {
    3
}
fn ctjb() -> SolutionType {
    4
}
fn zbbq() -> SolutionType {
    fsrf() * nsvg()
}
fn vmvh() -> SolutionType {
    phph() + ppsh()
}
fn mgzp() -> SolutionType {
    nmbj() * mrfl()
}
fn dfsn() -> SolutionType {
    4
}
fn cffj() -> SolutionType {
    szfn() * nhfn()
}
fn ddzj() -> SolutionType {
    1
}
fn btzf() -> SolutionType {
    vwtd() + wqnw()
}
fn fsjv() -> SolutionType {
    8
}
fn lrhm() -> SolutionType {
    4
}
fn crhb() -> SolutionType {
    bpbc() * vhwj()
}
fn ldrv() -> SolutionType {
    2
}
fn pqwq() -> SolutionType {
    prnb() * ghsg()
}
fn llrf() -> SolutionType {
    12
}
fn tlzg() -> SolutionType {
    14
}
fn qnml() -> SolutionType {
    qwbj() * rltv()
}
fn ctbb() -> SolutionType {
    9
}
fn hrnb() -> SolutionType {
    lvnh() * tgmh()
}
fn hvtz() -> SolutionType {
    mzbj() * gzwf()
}
fn hbqv() -> SolutionType {
    mjjv() + lmfl()
}
fn ggpz() -> SolutionType {
    12
}
fn bmch() -> SolutionType {
    2
}
fn sdjz() -> SolutionType {
    mspf() * rswd()
}
fn wjbl() -> SolutionType {
    3
}
fn mjqt() -> SolutionType {
    nrvn() + pzzl()
}
fn tmjg() -> SolutionType {
    dbgb() + chzq()
}
fn rfsq() -> SolutionType {
    ldtw() * dpwb()
}
fn zhfv() -> SolutionType {
    rsvr() * hfrg()
}
fn fddz() -> SolutionType {
    2
}
fn grcr() -> SolutionType {
    pcvz() + jfrl()
}
fn vmtv() -> SolutionType {
    ctjz() + mblw()
}
fn zznc() -> SolutionType {
    5
}
fn dsln() -> SolutionType {
    1
}
fn hprb() -> SolutionType {
    2
}
fn lrqp() -> SolutionType {
    fngd() * cdcv()
}
fn cnvf() -> SolutionType {
    jtfc() + nfvc()
}
fn nffp() -> SolutionType {
    2
}
fn znmj() -> SolutionType {
    2
}
fn cwlb() -> SolutionType {
    3
}
fn zmbn() -> SolutionType {
    2
}
fn nqwh() -> SolutionType {
    bllq() * ppqg()
}
fn bwcm() -> SolutionType {
    dhzq() + dgbj()
}
fn rgqr() -> SolutionType {
    20
}
fn qqdg() -> SolutionType {
    znjr() + bcjn()
}
fn jbpr() -> SolutionType {
    7
}
fn whcz() -> SolutionType {
    jtcl() + zscp()
}
fn hfgs() -> SolutionType {
    lnjr() * nrdn()
}
fn ssqt() -> SolutionType {
    hwts() * pbwn()
}
fn nhrv() -> SolutionType {
    wlqp() + bwsm()
}
fn cfhv() -> SolutionType {
    3
}
fn psgr() -> SolutionType {
    1
}
fn qvqf() -> SolutionType {
    thgv() + dhnm()
}
fn vlsw() -> SolutionType {
    5
}
fn lldv() -> SolutionType {
    5
}
fn rcmn() -> SolutionType {
    2
}
fn tcdm() -> SolutionType {
    19
}
fn rzmq() -> SolutionType {
    1
}
fn dprw() -> SolutionType {
    5
}
fn nhgv() -> SolutionType {
    4
}
fn tccp() -> SolutionType {
    2
}
fn tnmv() -> SolutionType {
    mrwd() * mzvz()
}
fn mdzl() -> SolutionType {
    7
}
fn vdzl() -> SolutionType {
    5
}
fn fbdc() -> SolutionType {
    2
}
fn dwvd() -> SolutionType {
    2
}
fn bpbc() -> SolutionType {
    plzd() * lggw()
}
fn wmmz() -> SolutionType {
    1
}
fn qwdp() -> SolutionType {
    zfbv() * gmjj()
}
fn gqrt() -> SolutionType {
    jbpr() * msjf()
}
fn wrsc() -> SolutionType {
    3
}
fn phml() -> SolutionType {
    hwtr() * dwvq()
}
fn crfq() -> SolutionType {
    smdq() * fczq()
}
fn vgwz() -> SolutionType {
    sgdg() * lqcf()
}
fn pbwn() -> SolutionType {
    5
}
fn jzwc() -> SolutionType {
    dvbw() / mvjf()
}
fn mzbj() -> SolutionType {
    8
}
fn crdf() -> SolutionType {
    fvvp() * pgdm()
}
fn wlqg() -> SolutionType {
    1
}
fn jnhs() -> SolutionType {
    3
}
fn bsww() -> SolutionType {
    1
}
fn qjgn() -> SolutionType {
    3
}
fn fspz() -> SolutionType {
    sppn() + gccm()
}
fn fsfs() -> SolutionType {
    2
}
fn scdb() -> SolutionType {
    cgdj() + psjm()
}
fn dbzd() -> SolutionType {
    gfbl() * bbdw()
}
fn wvmd() -> SolutionType {
    tzdh() * dmwv()
}
fn mlcf() -> SolutionType {
    3
}
fn dlqv() -> SolutionType {
    16
}
fn wptb() -> SolutionType {
    5
}
fn cpgj() -> SolutionType {
    zbbq() + sprb()
}
fn tcld() -> SolutionType {
    jqrj() * mfzt()
}
fn bhsg() -> SolutionType {
    ltbv() + jpnj()
}
fn sbpf() -> SolutionType {
    pwpf() / qnzc()
}
fn wtdm() -> SolutionType {
    2
}
fn ztgr() -> SolutionType {
    qqzh() * cpgj()
}
fn nsdh() -> SolutionType {
    4
}
fn glpv() -> SolutionType {
    gqzl() + qdgn()
}
fn scqh() -> SolutionType {
    5
}
fn rzhc() -> SolutionType {
    jdhj() + dhtm()
}
fn mzvb() -> SolutionType {
    8
}
fn dhtm() -> SolutionType {
    nfts() * gtgj()
}
fn mptr() -> SolutionType {
    whwc() / chmm()
}
fn pjsw() -> SolutionType {
    zcgf() * zngt()
}
fn qwtz() -> SolutionType {
    qrzb() * cqzp()
}
fn cjdc() -> SolutionType {
    2
}
fn tjlr() -> SolutionType {
    8
}
fn wlnn() -> SolutionType {
    phvm() / qmmc()
}
fn ccww() -> SolutionType {
    9
}
fn dhgn() -> SolutionType {
    2
}
fn zhhl() -> SolutionType {
    vzbp() * fsph()
}
fn lptf() -> SolutionType {
    dgdm() + jqdf()
}
fn hjlw() -> SolutionType {
    brcj() + lhcc()
}
fn vqcr() -> SolutionType {
    4
}
fn jrlh() -> SolutionType {
    2
}
fn pczh() -> SolutionType {
    frnp() - cndj()
}
fn mmsr() -> SolutionType {
    1
}
fn jvpl() -> SolutionType {
    3
}
fn jvqz() -> SolutionType {
    pbnq() + jnml()
}
fn rllf() -> SolutionType {
    2
}
fn cfvz() -> SolutionType {
    dbgc() * twsc()
}
fn mfmh() -> SolutionType {
    7
}
fn zcjg() -> SolutionType {
    2
}
fn stfd() -> SolutionType {
    5
}
fn hczv() -> SolutionType {
    2
}
fn mrmj() -> SolutionType {
    prnv() * bqrg()
}
fn gqsf() -> SolutionType {
    drmd() * brct()
}
fn gbsp() -> SolutionType {
    hdbp() * fndn()
}
fn ddwj() -> SolutionType {
    4
}
fn zdfh() -> SolutionType {
    6
}
fn zmjf() -> SolutionType {
    dchs() + fdfj()
}
fn qhvc() -> SolutionType {
    5
}
fn wqnw() -> SolutionType {
    sgjs() / flbl()
}
fn gmnd() -> SolutionType {
    pwbw() * fpgp()
}
fn nlwz() -> SolutionType {
    6
}
fn nhzn() -> SolutionType {
    12
}
fn bhhw() -> SolutionType {
    gjvw() * pcjn()
}
fn fcjr() -> SolutionType {
    2
}
fn fztr() -> SolutionType {
    2
}
fn rlrm() -> SolutionType {
    dqhs() * phml()
}
fn tzsc() -> SolutionType {
    whcz() * fmvb()
}
fn rssw() -> SolutionType {
    2
}
fn clqz() -> SolutionType {
    cqmb() + whnf()
}
fn scjr() -> SolutionType {
    3
}
fn tgbf() -> SolutionType {
    wwmt() * ltgr()
}
fn jgnt() -> SolutionType {
    vqrz() + hcfh()
}
fn mtts() -> SolutionType {
    mvjq() + bdzr()
}
fn hqls() -> SolutionType {
    10
}
fn qhlz() -> SolutionType {
    13
}
fn gqgz() -> SolutionType {
    5
}
fn cbmh() -> SolutionType {
    hgsh() * mgzp()
}
fn hczw() -> SolutionType {
    cqpc() - ztgr()
}
fn jtsf() -> SolutionType {
    2
}
fn sppn() -> SolutionType {
    rcnf() + rrvd()
}
fn csfd() -> SolutionType {
    bhrh() * mbnd()
}
fn tfgn() -> SolutionType {
    hrnb() * zdfm()
}
fn sgjf() -> SolutionType {
    fhhd() * zsrq()
}
fn svdl() -> SolutionType {
    2
}
fn ppfr() -> SolutionType {
    dfsn() * bhsg()
}
fn zrff() -> SolutionType {
    dstm() + bbrm()
}
fn tzdh() -> SolutionType {
    3
}
fn ppld() -> SolutionType {
    nszn() * szjv()
}
fn bdhg() -> SolutionType {
    nhrv() / rfnq()
}
fn npjd() -> SolutionType {
    brqs() * scqh()
}
fn flnz() -> SolutionType {
    gcpr() * qlpz()
}
fn wqtd() -> SolutionType {
    3
}
fn zbct() -> SolutionType {
    hljw() * jnsz()
}
fn jvnn() -> SolutionType {
    3
}
fn srws() -> SolutionType {
    2
}
fn qrmh() -> SolutionType {
    tfln() - sflj()
}
fn dqtn() -> SolutionType {
    5
}
fn mnjg() -> SolutionType {
    wmhn() * dcbt()
}
fn rdvm() -> SolutionType {
    hgrl() * fbmd()
}
fn grtv() -> SolutionType {
    wvmd() + qbzt()
}
fn gwgb() -> SolutionType {
    bsrv() / mghq()
}
fn sjtt() -> SolutionType {
    13
}
fn hgrl() -> SolutionType {
    5
}
fn bdzr() -> SolutionType {
    mnqb() + gsps()
}
fn twsc() -> SolutionType {
    zccz() * ffsp()
}
fn spmz() -> SolutionType {
    sczz() - zgpt()
}
fn mqmn() -> SolutionType {
    pcpq() + lgwd()
}
fn pdsm() -> SolutionType {
    2
}
fn qgsh() -> SolutionType {
    vmvh() - bsss()
}
fn vptr() -> SolutionType {
    2
}
fn tvgn() -> SolutionType {
    3
}
fn qqzh() -> SolutionType {
    wbzd() * hthb()
}
fn ffnm() -> SolutionType {
    zbrc() / rsqm()
}
fn gqtb() -> SolutionType {
    cvpm() + schq()
}
fn hcrf() -> SolutionType {
    2
}
fn hhhl() -> SolutionType {
    ncmd() + mptr()
}
fn rpgm() -> SolutionType {
    1
}
fn fqpg() -> SolutionType {
    2
}
fn lwrl() -> SolutionType {
    sbsn() * cmwj()
}
fn plzd() -> SolutionType {
    3
}
fn nwzz() -> SolutionType {
    4
}
fn ncqs() -> SolutionType {
    zpqp() * cmch()
}
fn mvjq() -> SolutionType {
    20
}
fn bslh() -> SolutionType {
    ndvn() + qvhw()
}
fn thsh() -> SolutionType {
    lhrv() - czwt()
}
fn zbjp() -> SolutionType {
    gwgb() / mwsp()
}
fn wblv() -> SolutionType {
    3
}
fn nbfp() -> SolutionType {
    2
}
fn phqn() -> SolutionType {
    twnv() * jpqg()
}
fn lmfl() -> SolutionType {
    hqls() + mvls()
}
fn dgjd() -> SolutionType {
    dgrd() + crhb()
}
fn mbgh() -> SolutionType {
    nrhz() * vlfc()
}
fn dqhs() -> SolutionType {
    5
}
fn rgrb() -> SolutionType {
    11
}
fn nrhz() -> SolutionType {
    llbt() + ztfs()
}
fn qrvm() -> SolutionType {
    7
}
fn zrfj() -> SolutionType {
    hrjs() + wcws()
}
fn wztc() -> SolutionType {
    wjjg() + bvsw()
}
fn bttz() -> SolutionType {
    2
}
fn vzcp() -> SolutionType {
    1
}
fn pwgs() -> SolutionType {
    pwlc() * vdzl()
}
fn lpqf() -> SolutionType {
    zljz() + lrnc()
}
fn vvnn() -> SolutionType {
    3
}
fn jjbn() -> SolutionType {
    4
}
fn ngtp() -> SolutionType {
    glml() * nfds()
}
fn llhf() -> SolutionType {
    2
}
fn stpm() -> SolutionType {
    6
}
fn wnst() -> SolutionType {
    3
}
fn pwvr() -> SolutionType {
    1
}
fn flbv() -> SolutionType {
    vgwv() - ttfq()
}
fn mvsl() -> SolutionType {
    spmz() + whlr()
}
fn rblr() -> SolutionType {
    5
}
fn twbm() -> SolutionType {
    3
}
fn smvw() -> SolutionType {
    15
}
fn njsj() -> SolutionType {
    4
}
fn vvdm() -> SolutionType {
    qwdp() * prqf()
}
fn rths() -> SolutionType {
    pqlz() * shnl()
}
fn fndn() -> SolutionType {
    2
}
fn qvtz() -> SolutionType {
    3
}
fn btwc() -> SolutionType {
    gbpr() * dmbf()
}
fn thqw() -> SolutionType {
    2
}
fn hgph() -> SolutionType {
    13
}
fn ljsr() -> SolutionType {
    10
}
fn vphq() -> SolutionType {
    7
}
fn hcnn() -> SolutionType {
    crrb() + hzvd()
}
fn rcjm() -> SolutionType {
    2
}
fn ffss() -> SolutionType {
    pcgs() * bscg()
}
fn jpnj() -> SolutionType {
    wfnh() + njzd()
}
fn zzml() -> SolutionType {
    2
}
fn sbng() -> SolutionType {
    dlqv() + smvw()
}
fn mfqg() -> SolutionType {
    5
}
fn nszn() -> SolutionType {
    zlrf() + dqzg()
}
fn sqbc() -> SolutionType {
    4
}
fn pzrl() -> SolutionType {
    3
}
fn mnzd() -> SolutionType {
    5
}
fn sdhf() -> SolutionType {
    2
}
fn jcmp() -> SolutionType {
    cjgg() * wnst()
}
fn qlzn() -> SolutionType {
    thwd() - mvcc()
}
fn dlmg() -> SolutionType {
    frbg() + zsdd()
}
fn mjbj() -> SolutionType {
    15
}
fn mnqb() -> SolutionType {
    14
}
fn vvml() -> SolutionType {
    htsg() + tpdl()
}
fn mwsp() -> SolutionType {
    2
}
fn dzsr() -> SolutionType {
    rths() * dprw()
}
fn dtbh() -> SolutionType {
    qttw() * dszc()
}
fn ctqm() -> SolutionType {
    4
}
fn dnqs() -> SolutionType {
    17
}
fn gbdb() -> SolutionType {
    3
}
fn qvhw() -> SolutionType {
    bjbh() * gvwj()
}
fn bwqc() -> SolutionType {
    4
}
fn jgdn() -> SolutionType {
    nqcl() + dhmt()
}
fn nrmd() -> SolutionType {
    tjcw() * cdsl()
}
fn zzjv() -> SolutionType {
    rbhw() * mgwv()
}
fn rrpz() -> SolutionType {
    7
}
fn zvjz() -> SolutionType {
    zcjg() * mvzs()
}
fn tlrn() -> SolutionType {
    zmsf() * jfzs()
}
fn ztrc() -> SolutionType {
    1
}
fn pcvz() -> SolutionType {
    2
}
fn hftm() -> SolutionType {
    bdhg() - phpz()
}
fn hspg() -> SolutionType {
    3
}
fn jczt() -> SolutionType {
    3
}
fn zngt() -> SolutionType {
    gmnd() + bnwg()
}
fn ncrp() -> SolutionType {
    ztnw() * tmjg()
}
fn vhwj() -> SolutionType {
    3
}
fn ldbn() -> SolutionType {
    ddfp() + rwjv()
}
fn gzrp() -> SolutionType {
    3
}
fn jlzr() -> SolutionType {
    2
}
fn jfvl() -> SolutionType {
    2
}
fn mrvr() -> SolutionType {
    bhcz() * ffcl()
}
fn zqdh() -> SolutionType {
    flwb() + jnbr()
}
fn tcpw() -> SolutionType {
    jffs() - hwrm()
}
fn gfbl() -> SolutionType {
    2
}
fn fccf() -> SolutionType {
    3
}
fn flwb() -> SolutionType {
    2
}
fn dthl() -> SolutionType {
    7
}
fn gccm() -> SolutionType {
    llgd() * zpwd()
}
fn wdrz() -> SolutionType {
    sqlt() + cdrg()
}
fn hcqp() -> SolutionType {
    17
}
fn lwtg() -> SolutionType {
    zmbn() * cqcj()
}
fn nnff() -> SolutionType {
    mwgd() + rczp()
}
fn ptzv() -> SolutionType {
    qmsn() + rlqf()
}
fn rpth() -> SolutionType {
    ddts() * gzgv()
}
fn ljjn() -> SolutionType {
    7
}
fn ttdf() -> SolutionType {
    1
}
fn szdc() -> SolutionType {
    czlz() * nrdf()
}
fn hbmm() -> SolutionType {
    dzsr() / zrgc()
}
fn lbzj() -> SolutionType {
    4
}
fn hbnn() -> SolutionType {
    3
}
fn qhtt() -> SolutionType {
    qqpp() * pgmh()
}
fn gvlr() -> SolutionType {
    7
}
fn pjgq() -> SolutionType {
    qvtz() * hbtr()
}
fn bdvh() -> SolutionType {
    wdjs() - zhdz()
}
fn jzzf() -> SolutionType {
    7
}
fn wzmd() -> SolutionType {
    13
}
fn jlwd() -> SolutionType {
    mrvr() + gdmv()
}
fn pptt() -> SolutionType {
    dlbj() * gggc()
}
fn gzrs() -> SolutionType {
    2
}
fn hsmj() -> SolutionType {
    zczb() * rcjm()
}
fn csfb() -> SolutionType {
    cngl() + rrpz()
}
fn hcbz() -> SolutionType {
    lqqs() + cbgz()
}
fn brsf() -> SolutionType {
    lssl() + tzgg()
}
fn vsjn() -> SolutionType {
    lphc() * nvhj()
}
fn dchs() -> SolutionType {
    jcmp() + qdgf()
}
fn vrjj() -> SolutionType {
    2
}
fn bjbh() -> SolutionType {
    csfd() - nrlg()
}
fn drnl() -> SolutionType {
    jznt() / snjp()
}
fn jfbp() -> SolutionType {
    2
}
fn bqrq() -> SolutionType {
    czjv() * bpzt()
}
fn mmgl() -> SolutionType {
    wjtj() * sgnh()
}
fn grgr() -> SolutionType {
    mwhz() * dtbh()
}
fn bbwz() -> SolutionType {
    6
}
fn dwmw() -> SolutionType {
    4
}
fn cfhp() -> SolutionType {
    4
}
fn djwl() -> SolutionType {
    3
}
fn twlj() -> SolutionType {
    mrdj() / ngwj()
}
fn srnd() -> SolutionType {
    glnt() * jgnt()
}
fn lmgj() -> SolutionType {
    tzqc() + nrmd()
}
fn jznt() -> SolutionType {
    gnth() * jndn()
}
fn czhj() -> SolutionType {
    hrfv() * bccv()
}
fn ssbn() -> SolutionType {
    qswz() / whrf()
}
fn mzwb() -> SolutionType {
    dlmg() + vqbr()
}
fn wqmn() -> SolutionType {
    sdjz() + rdjh()
}
fn cfqn() -> SolutionType {
    5
}
fn wrlq() -> SolutionType {
    lrhm() * tmvq()
}
fn root() -> SolutionType {
    dbcq() + zmvq()
}
fn dszc() -> SolutionType {
    17
}
fn gzdv() -> SolutionType {
    2
}
fn mrgs() -> SolutionType {
    3
}
fn zqnz() -> SolutionType {
    zlzc() * wlnq()
}
fn plns() -> SolutionType {
    4
}
fn gqpp() -> SolutionType {
    13
}
fn fwwb() -> SolutionType {
    3
}
fn rvct() -> SolutionType {
    wzdw() + psgr()
}
fn fjnh() -> SolutionType {
    5
}
fn dcbt() -> SolutionType {
    5
}
fn twqw() -> SolutionType {
    17
}
fn whlr() -> SolutionType {
    wrwt() * mlqw()
}
fn tgds() -> SolutionType {
    zjdj() * sdqn()
}
fn lzzq() -> SolutionType {
    5
}
fn spbb() -> SolutionType {
    1
}
fn bvsw() -> SolutionType {
    wrgd() * mbsc()
}
fn rwnw() -> SolutionType {
    2
}
fn rpnz() -> SolutionType {
    bstt() * fvbg()
}
fn jwzt() -> SolutionType {
    rjzt() * rlmj()
}
fn hqsr() -> SolutionType {
    gnbd() * hnrl()
}
fn fbfl() -> SolutionType {
    mvsn() * mcdw()
}
fn chvj() -> SolutionType {
    5
}
fn cfwm() -> SolutionType {
    hsdn() / bpwg()
}
fn blch() -> SolutionType {
    3
}
fn vjdl() -> SolutionType {
    mzgv() - qlrl()
}
fn gblf() -> SolutionType {
    dddl() + sjfm()
}
fn bnhz() -> SolutionType {
    bpnc() + qgnm()
}
fn chtc() -> SolutionType {
    3
}
fn sbjl() -> SolutionType {
    3
}
fn tpfw() -> SolutionType {
    bchq() + fhfv()
}
fn sgmz() -> SolutionType {
    2
}
fn gcdw() -> SolutionType {
    2
}
fn zfbv() -> SolutionType {
    2
}
fn rmqc() -> SolutionType {
    wblq() * bjcd()
}
fn ldtw() -> SolutionType {
    pcnn() + mrfp()
}
fn bqlz() -> SolutionType {
    3
}
fn qvvs() -> SolutionType {
    mdqz() + tjcf()
}
fn rwnf() -> SolutionType {
    10
}
fn vggv() -> SolutionType {
    3
}
fn gjtv() -> SolutionType {
    pjvd() + zmjf()
}
fn zpzf() -> SolutionType {
    4
}
fn qrjg() -> SolutionType {
    17
}
fn pmjz() -> SolutionType {
    drnl() - dsmw()
}
fn bsjn() -> SolutionType {
    5
}
fn fdvc() -> SolutionType {
    2
}
fn czlc() -> SolutionType {
    5
}
fn ddfp() -> SolutionType {
    4
}
fn npfl() -> SolutionType {
    2
}
fn ndlz() -> SolutionType {
    vzrv() * zspz()
}
fn pjhg() -> SolutionType {
    3
}
fn tnrg() -> SolutionType {
    1
}
fn vlmb() -> SolutionType {
    2
}
fn wdlb() -> SolutionType {
    1
}
fn pgmh() -> SolutionType {
    tfgn() + lmgj()
}
fn cqds() -> SolutionType {
    2
}
fn grdf() -> SolutionType {
    fmng() + njjs()
}
fn trzs() -> SolutionType {
    3
}
fn fhzg() -> SolutionType {
    2
}
fn wvbg() -> SolutionType {
    ptzv() + mnzd()
}
fn pwbw() -> SolutionType {
    3
}
fn tpqn() -> SolutionType {
    qqhj() + hjlw()
}
fn jnbr() -> SolutionType {
    dntw() + zsnp()
}
fn vqrz() -> SolutionType {
    qwjc() * zhhh()
}
fn smdq() -> SolutionType {
    3
}
fn fwht() -> SolutionType {
    vhzm() + zcld()
}
fn qzhh() -> SolutionType {
    2
}
fn wcrq() -> SolutionType {
    5
}
fn zbzt() -> SolutionType {
    mvdv() * qvsg()
}
fn bbqm() -> SolutionType {
    13
}
fn phpz() -> SolutionType {
    cqqw() + ffnm()
}
fn swjw() -> SolutionType {
    2
}
fn djjq() -> SolutionType {
    hfgs() + tzmw()
}
fn qhcd() -> SolutionType {
    5
}
fn dzft() -> SolutionType {
    vsjn() - cqrp()
}
fn qsrl() -> SolutionType {
    2
}
fn sqms() -> SolutionType {
    bfvp() + rnjb()
}
fn rdtl() -> SolutionType {
    prpb() * nfwg()
}
fn hpct() -> SolutionType {
    zcvd() * hwwb()
}
fn pjvd() -> SolutionType {
    rfqt() * sjqz()
}
fn zsng() -> SolutionType {
    twhf() + vqfz()
}
fn grft() -> SolutionType {
    8
}
fn dlvl() -> SolutionType {
    nwzz() + gbsp()
}
fn cjgg() -> SolutionType {
    rvcc() * bhjf()
}
fn tzjw() -> SolutionType {
    rsgn() - lncn()
}
fn zspz() -> SolutionType {
    dzrg() + vdpb()
}
fn cqqw() -> SolutionType {
    ljmw() * hrgz()
}
fn nvhj() -> SolutionType {
    fvfz() * bttz()
}
fn nrms() -> SolutionType {
    2
}
fn srwt() -> SolutionType {
    5
}
fn mfbb() -> SolutionType {
    2
}
fn vqth() -> SolutionType {
    mlcf() + nzmz()
}
fn hwts() -> SolutionType {
    17
}
fn gcpr() -> SolutionType {
    tnrg() + crfq()
}
fn ffll() -> SolutionType {
    5
}
fn wblz() -> SolutionType {
    2
}
fn jqmw() -> SolutionType {
    5
}
fn wrgd() -> SolutionType {
    7
}
fn qhdg() -> SolutionType {
    fglf() * qvvs()
}
fn cgdj() -> SolutionType {
    10
}
fn rvgc() -> SolutionType {
    qdrw() + msnd()
}
fn hsbs() -> SolutionType {
    lhvs() * lhtz()
}
fn qnhp() -> SolutionType {
    ftzt() + nwgg()
}
fn blws() -> SolutionType {
    fzjd() / rvct()
}
fn frbf() -> SolutionType {
    dlvl() / rcmn()
}
fn gtdp() -> SolutionType {
    3
}
fn tnsp() -> SolutionType {
    2
}
fn hwbq() -> SolutionType {
    18
}
fn ccdj() -> SolutionType {
    cbzl() + dclg()
}
fn ndtn() -> SolutionType {
    ntcc() + ptdp()
}
fn jfrl() -> SolutionType {
    rqwp() + dlbp()
}
fn dmwr() -> SolutionType {
    ffll() * tnwd()
}
fn hzvf() -> SolutionType {
    qgrh() + fmfj()
}
fn zwrj() -> SolutionType {
    2
}
fn mbnd() -> SolutionType {
    nhzn() - bggd()
}
fn ffbs() -> SolutionType {
    3
}
fn vrsh() -> SolutionType {
    cghd() * fwlh()
}
fn pfcj() -> SolutionType {
    3
}
fn jvlw() -> SolutionType {
    qfgs() * pfcj()
}
fn vpdp() -> SolutionType {
    15
}
fn fcvl() -> SolutionType {
    4
}
fn dqzg() -> SolutionType {
    11
}
fn szvj() -> SolutionType {
    2
}
fn nsjv() -> SolutionType {
    3
}
fn qgrh() -> SolutionType {
    1
}
fn rbjd() -> SolutionType {
    4
}
fn bjcd() -> SolutionType {
    3
}
fn spdg() -> SolutionType {
    jtzs() + fzmh()
}
fn dsrb() -> SolutionType {
    2
}
fn brcj() -> SolutionType {
    14
}
fn tcmb() -> SolutionType {
    12
}
fn zrpz() -> SolutionType {
    11
}
fn wqdv() -> SolutionType {
    scgt() * qbqh()
}
fn ldjc() -> SolutionType {
    3
}
fn qgcr() -> SolutionType {
    7
}
fn pwtd() -> SolutionType {
    rzmq() + lljz()
}
fn whwc() -> SolutionType {
    wjwq() + fcts()
}
fn schq() -> SolutionType {
    rwgg() * zdcc()
}
fn vmdq() -> SolutionType {
    2
}
fn vtnj() -> SolutionType {
    1
}
fn wscf() -> SolutionType {
    bnff() * wqtd()
}
fn wsnp() -> SolutionType {
    9
}
fn zrcm() -> SolutionType {
    qslc() * qndn()
}
fn bsgl() -> SolutionType {
    2
}
fn gpmd() -> SolutionType {
    mzdz() * vnzf()
}
fn gvjd() -> SolutionType {
    2
}
fn mngq() -> SolutionType {
    hfsz() + wmmz()
}
fn hssq() -> SolutionType {
    mvwb() * jlwz()
}
fn bwsm() -> SolutionType {
    dhvn() + mhvw()
}
fn vdgg() -> SolutionType {
    szdc() * pszf()
}
fn htlp() -> SolutionType {
    dhpp() / stdr()
}
fn wrwt() -> SolutionType {
    mrhl() * rqhj()
}
fn mchw() -> SolutionType {
    4
}
fn qrzb() -> SolutionType {
    btdm() + bqrq()
}
fn scgt() -> SolutionType {
    2
}
fn bhjf() -> SolutionType {
    3
}
fn wqtw() -> SolutionType {
    fhpr() + gfdp()
}
fn rvjn() -> SolutionType {
    2
}
fn dzrg() -> SolutionType {
    fflf() + qzjs()
}
fn ggvj() -> SolutionType {
    3
}
fn gtgs() -> SolutionType {
    5
}
fn snjp() -> SolutionType {
    2
}
fn sqlt() -> SolutionType {
    8
}
fn pszf() -> SolutionType {
    3
}
fn whrf() -> SolutionType {
    2
}
fn phrp() -> SolutionType {
    2
}
fn cdhg() -> SolutionType {
    12
}
fn dvbw() -> SolutionType {
    zrff() * gtzf()
}
fn lssl() -> SolutionType {
    16
}
fn djln() -> SolutionType {
    qprb() - ncwt()
}
fn pfqg() -> SolutionType {
    ftfh() + hbmm()
}
fn mwhm() -> SolutionType {
    11
}
fn jjpg() -> SolutionType {
    nwgv() * lqmh()
}
fn cdrg() -> SolutionType {
    10
}
fn bmwh() -> SolutionType {
    4
}
fn cbrw() -> SolutionType {
    4
}
fn dmwv() -> SolutionType {
    qgcr() + rngp()
}
fn jfzt() -> SolutionType {
    3
}
fn bqrg() -> SolutionType {
    zbzt() / thqw()
}
fn twtl() -> SolutionType {
    dzdd() * scdb()
}
fn jrjp() -> SolutionType {
    8
}
fn zjwp() -> SolutionType {
    16
}
fn chmm() -> SolutionType {
    2
}
fn lpms() -> SolutionType {
    dwvd() * bcmf()
}
fn gprr() -> SolutionType {
    zbqv() + hzgt()
}
fn znjg() -> SolutionType {
    wmrp() * ldrv()
}
fn qmsn() -> SolutionType {
    5
}
fn phlv() -> SolutionType {
    wscf() + bpjm()
}
fn rwjv() -> SolutionType {
    3
}
fn pqzt() -> SolutionType {
    lpvd() + djcm()
}
fn sgrd() -> SolutionType {
    cqrb() - zvpd()
}
fn ggsl() -> SolutionType {
    zprb() * pzvn()
}
fn vjcd() -> SolutionType {
    gdvt() * sznf()
}
fn dhzq() -> SolutionType {
    gqwl() * vjlh()
}
fn wlqp() -> SolutionType {
    ldfw() / hfjm()
}
fn bcsr() -> SolutionType {
    2
}
fn zgzb() -> SolutionType {
    8
}
fn jnvb() -> SolutionType {
    5
}
fn fshw() -> SolutionType {
    mlhp() / hdmg()
}
fn slmh() -> SolutionType {
    hswt() + jgdn()
}
fn hqsf() -> SolutionType {
    5
}
fn zpnw() -> SolutionType {
    jfzt() * drtf()
}
fn qshb() -> SolutionType {
    lgjv() - wmnp()
}
fn qvct() -> SolutionType {
    3
}
fn tqhf() -> SolutionType {
    3
}
fn vrgj() -> SolutionType {
    3
}
fn njjs() -> SolutionType {
    hgph() + wmpf()
}
fn cqpc() -> SolutionType {
    rvzz() / dggd()
}
fn qslc() -> SolutionType {
    npwg() + ctcj()
}
fn rswd() -> SolutionType {
    13
}
fn slqb() -> SolutionType {
    3
}
fn vfjc() -> SolutionType {
    fzdr() + nrrs()
}
fn vcqv() -> SolutionType {
    5
}
fn ccwd() -> SolutionType {
    rwzn() * bwcm()
}
fn llwb() -> SolutionType {
    hlnd() * ptrt()
}
fn bdsq() -> SolutionType {
    3
}
fn ndhv() -> SolutionType {
    2
}
fn rhhz() -> SolutionType {
    12
}
fn qmmc() -> SolutionType {
    3
}
fn hjvb() -> SolutionType {
    4
}
fn tcdj() -> SolutionType {
    17
}
fn vzbp() -> SolutionType {
    3
}
fn tpdj() -> SolutionType {
    qqlw() - ztrc()
}
fn lngh() -> SolutionType {
    2
}
fn dplg() -> SolutionType {
    8
}
fn bwch() -> SolutionType {
    dwwh() + qjvs()
}
fn ssrr() -> SolutionType {
    10
}
fn ngtq() -> SolutionType {
    6
}
fn bhrh() -> SolutionType {
    3
}
fn pqlz() -> SolutionType {
    wbnj() * wzwd()
}
fn rnjb() -> SolutionType {
    4
}
fn sdqn() -> SolutionType {
    4
}
fn dtjn() -> SolutionType {
    chbf() + ggqb()
}
fn wqnv() -> SolutionType {
    3
}
fn drlw() -> SolutionType {
    wcvv() + jmlr()
}
fn gqnt() -> SolutionType {
    qhcz() * tctm()
}
fn jdgz() -> SolutionType {
    nwcj() * szsv()
}
fn zhzw() -> SolutionType {
    2
}
fn zwgp() -> SolutionType {
    2
}
fn bvdc() -> SolutionType {
    3
}
fn rtrc() -> SolutionType {
    3
}
fn jwbg() -> SolutionType {
    hsmj() - vdlz()
}
fn szqq() -> SolutionType {
    vggv() * cwlb()
}
fn ntpp() -> SolutionType {
    mlpv() * bvht()
}
fn hcsb() -> SolutionType {
    2
}
fn fwpp() -> SolutionType {
    fqsm() * mzvj()
}
fn vbbs() -> SolutionType {
    4
}
fn dglt() -> SolutionType {
    mwtw() + jjbn()
}
fn dmmt() -> SolutionType {
    thsh() * wrsc()
}
fn bpjm() -> SolutionType {
    grcf() * rwwm()
}
fn wjjg() -> SolutionType {
    fzdn() + gjwh()
}
fn qqlw() -> SolutionType {
    nsdh() + fcvl()
}
fn fwss() -> SolutionType {
    wllz() * swnn()
}
fn ztht() -> SolutionType {
    vsmn() * bfpt()
}
fn mtrq() -> SolutionType {
    ffss() / qhvc()
}
fn vblq() -> SolutionType {
    6
}
fn shnl() -> SolutionType {
    2
}
fn sdrq() -> SolutionType {
    3
}
fn gwbj() -> SolutionType {
    7
}
fn vnbp() -> SolutionType {
    2
}
fn jtzc() -> SolutionType {
    fpjv() * tnjj()
}
fn wpbt() -> SolutionType {
    sgrd() + twtl()
}
fn wdmc() -> SolutionType {
    flnz() * sfpd()
}
fn vdvn() -> SolutionType {
    ddvr() + rjbf()
}
fn stdr() -> SolutionType {
    2
}
fn sjtc() -> SolutionType {
    2
}
fn tvpb() -> SolutionType {
    2
}
fn jnzv() -> SolutionType {
    5
}
fn prnv() -> SolutionType {
    3
}
fn zdsl() -> SolutionType {
    hgsb() * mcvf()
}
fn rlmd() -> SolutionType {
    vqth() + hnrm()
}
fn hdgm() -> SolutionType {
    dnhg() - fdgq()
}
fn ctzw() -> SolutionType {
    bsww() + wrhn()
}
fn dqdg() -> SolutionType {
    bcdw() * rblr()
}
fn wqhs() -> SolutionType {
    njhj() + llrf()
}
fn zcvd() -> SolutionType {
    3
}
fn whlf() -> SolutionType {
    4
}
fn fwlh() -> SolutionType {
    5
}
fn zfbp() -> SolutionType {
    3
}
fn nwhn() -> SolutionType {
    6
}
fn rgbg() -> SolutionType {
    qrmh() - dwhs()
}
fn dbfq() -> SolutionType {
    rcmz() + pstz()
}
fn rcfj() -> SolutionType {
    3
}
fn scbt() -> SolutionType {
    2
}
fn prnb() -> SolutionType {
    9
}
fn zjdj() -> SolutionType {
    ggsl() / nnth()
}
fn qjvs() -> SolutionType {
    cvlt() / zbgs()
}
fn gdrr() -> SolutionType {
    11
}
fn hcrq() -> SolutionType {
    4
}
fn pbzh() -> SolutionType {
    4
}
fn cdjb() -> SolutionType {
    bmch() + pqwq()
}
fn nfvc() -> SolutionType {
    2
}
fn wczz() -> SolutionType {
    2
}
fn fsph() -> SolutionType {
    5
}
fn bscg() -> SolutionType {
    5
}
fn lppj() -> SolutionType {
    blws() + tgvz()
}
fn tsvp() -> SolutionType {
    lzrf() / qqdb()
}
fn fdsf() -> SolutionType {
    cfsp() + srws()
}
fn mlfb() -> SolutionType {
    sgqs() * vnbc()
}
fn whgc() -> SolutionType {
    cqds() * bhqh()
}
fn lhrv() -> SolutionType {
    rdsh() * nsdq()
}
fn mlhp() -> SolutionType {
    sjqv() * ftcp()
}
fn njhj() -> SolutionType {
    qjgp() + hpbd()
}
fn gcdm() -> SolutionType {
    2
}
fn rjbf() -> SolutionType {
    zjwp() * rjcw()
}
fn nrsb() -> SolutionType {
    5
}
fn brln() -> SolutionType {
    fcjr() * ptqm()
}
fn dlbp() -> SolutionType {
    5
}
fn dvzd() -> SolutionType {
    brsf() * gcdm()
}
fn jnml() -> SolutionType {
    11
}
fn vjlh() -> SolutionType {
    12
}
fn dntw() -> SolutionType {
    cjbj() * dshg()
}
fn bfjm() -> SolutionType {
    nffp() * snpf()
}
fn smqn() -> SolutionType {
    4
}
fn rnpg() -> SolutionType {
    wnvp() + mtbg()
}
fn pftp() -> SolutionType {
    hsbs() + vgwz()
}
fn znzh() -> SolutionType {
    mcpq() * dqln()
}
fn mzvs() -> SolutionType {
    1
}
fn lwzj() -> SolutionType {
    4
}
fn cjbj() -> SolutionType {
    pwzr() + sfvc()
}
fn zgnh() -> SolutionType {
    3
}
fn pcgs() -> SolutionType {
    vnct() + wdnq()
}
fn bftj() -> SolutionType {
    5
}
fn mqll() -> SolutionType {
    1
}
fn tmvq() -> SolutionType {
    4
}
fn pwpf() -> SolutionType {
    gvqs() + jlvr()
}
fn bvht() -> SolutionType {
    3
}
fn sjpv() -> SolutionType {
    3
}
fn pqrf() -> SolutionType {
    3
}
fn psjm() -> SolutionType {
    jnzv() * pvww()
}
fn pqhz() -> SolutionType {
    lzpg() / tbls()
}
fn bsrv() -> SolutionType {
    ttrq() * pqwg()
}
fn cmch() -> SolutionType {
    fvsv() * tcdj()
}
fn jmls() -> SolutionType {
    3
}
fn rcbn() -> SolutionType {
    mbnq() + jnhd()
}
fn gdmv() -> SolutionType {
    cvnn() - pqcl()
}
fn bqzn() -> SolutionType {
    6
}
fn rjhc() -> SolutionType {
    4
}
fn mtpq() -> SolutionType {
    2
}
fn cvlt() -> SolutionType {
    hcrj() + cfhp()
}
fn vdpb() -> SolutionType {
    grzf() + djfl()
}
fn lnjr() -> SolutionType {
    lrfs() / jmls()
}
fn bqzs() -> SolutionType {
    mjbj() + qrvm()
}
fn vmbv() -> SolutionType {
    zpff() + vtlz()
}
fn vlcb() -> SolutionType {
    jpth() + fsrh()
}
fn wddr() -> SolutionType {
    zcpn() * czhj()
}
fn ffpr() -> SolutionType {
    3
}
fn qllt() -> SolutionType {
    4
}
fn cnsb() -> SolutionType {
    1
}
fn zwnm() -> SolutionType {
    ssqt() + prmq()
}
fn qbph() -> SolutionType {
    4
}
fn hcvc() -> SolutionType {
    nhrp() * bbll()
}
fn qtgs() -> SolutionType {
    dnqs() * cdmz()
}
fn zljz() -> SolutionType {
    1
}
fn vprh() -> SolutionType {
    8
}
fn snzt() -> SolutionType {
    2
}
fn jtbg() -> SolutionType {
    zhzw() * qvqf()
}
fn ccps() -> SolutionType {
    2
}
fn jlvr() -> SolutionType {
    1
}
fn brpt() -> SolutionType {
    wgsj() + grdf()
}
fn bgct() -> SolutionType {
    2
}
fn sljm() -> SolutionType {
    5
}
fn wdnq() -> SolutionType {
    vgqd() * rpjc()
}
fn nmgj() -> SolutionType {
    dglt() + ppnn()
}
fn qqpp() -> SolutionType {
    2
}
fn hfsz() -> SolutionType {
    jwrj() * vmzb()
}
fn nwwm() -> SolutionType {
    fqhq() * pzqq()
}
fn cpcf() -> SolutionType {
    2
}
fn nchn() -> SolutionType {
    4
}
fn jtzs() -> SolutionType {
    lmvz() * mtrq()
}
fn qttw() -> SolutionType {
    3
}
fn llzt() -> SolutionType {
    gvbg() * twds()
}
fn mgcv() -> SolutionType {
    dhgn() * gmnw()
}
fn fmcd() -> SolutionType {
    12
}
fn qncj() -> SolutionType {
    svdl() * ffnp()
}
fn hqjm() -> SolutionType {
    znmj() * ffpr()
}
fn bqbr() -> SolutionType {
    8
}
fn ndcf() -> SolutionType {
    1
}
fn wzwd() -> SolutionType {
    3
}
fn sjlh() -> SolutionType {
    qpqz() * lfzg()
}
fn pdmd() -> SolutionType {
    qzrc() + clvv()
}
fn ssfg() -> SolutionType {
    mtpq() + hjvb()
}
fn tcdt() -> SolutionType {
    4
}
fn sjqz() -> SolutionType {
    5
}
fn gnbd() -> SolutionType {
    2
}
fn blrl() -> SolutionType {
    rrzl() * jlfd()
}
fn lfhq() -> SolutionType {
    srwt() * plwr()
}
fn hmzh() -> SolutionType {
    7
}
fn rbdl() -> SolutionType {
    13
}
fn fhgj() -> SolutionType {
    2
}
fn snwr() -> SolutionType {
    vjcd() * bnbt()
}
fn ttmp() -> SolutionType {
    pwjt() + mzvb()
}
fn nfwg() -> SolutionType {
    3
}
fn lnhj() -> SolutionType {
    vdhd() + fshw()
}
fn qprb() -> SolutionType {
    vvzv() * sqzv()
}
fn pwzr() -> SolutionType {
    2
}
fn rsgc() -> SolutionType {
    cbrw() + wqcj()
}
fn jtcl() -> SolutionType {
    ddwj() + wqnv()
}
fn qrhp() -> SolutionType {
    qnml() - szsn()
}
fn gffs() -> SolutionType {
    5
}
fn cmwj() -> SolutionType {
    pgvc() + wvjf()
}
fn jrvd() -> SolutionType {
    tfdg() * fwwb()
}
fn glwz() -> SolutionType {
    sppw() * zwgp()
}
fn lphc() -> SolutionType {
    4
}
fn tdhd() -> SolutionType {
    frbf() + mrgs()
}
fn mzsw() -> SolutionType {
    spbb() + hrbj()
}
fn prfn() -> SolutionType {
    jmbw() * ltsj()
}
fn fvmq() -> SolutionType {
    10
}
fn fgjv() -> SolutionType {
    4
}
fn wjtj() -> SolutionType {
    7
}
fn hfrb() -> SolutionType {
    nwhd() + gjzd()
}
fn ztgg() -> SolutionType {
    qhtt() / nfhl()
}
fn szsn() -> SolutionType {
    3
}
fn vvvg() -> SolutionType {
    6
}
fn fvdc() -> SolutionType {
    rvqd() + bbwz()
}
fn zqcj() -> SolutionType {
    5
}
fn rvjs() -> SolutionType {
    18
}
fn wnvp() -> SolutionType {
    qlzn() * qprg()
}
fn hbcj() -> SolutionType {
    2
}
fn clvv() -> SolutionType {
    3
}
fn lttb() -> SolutionType {
    1
}
fn gqwl() -> SolutionType {
    cjdg() * tjlr()
}
fn gjzd() -> SolutionType {
    9
}
fn hvvc() -> SolutionType {
    lbzj() * fdvc()
}
fn nnth() -> SolutionType {
    3
}
fn dstj() -> SolutionType {
    17
}
fn tvbr() -> SolutionType {
    phft() - cvfh()
}
fn lzzf() -> SolutionType {
    cpsb() * slmh()
}
fn tbgt() -> SolutionType {
    5
}
fn dggd() -> SolutionType {
    3
}
fn mnmz() -> SolutionType {
    10
}
fn rtsw() -> SolutionType {
    nzpv() - vczf()
}
fn gvnr() -> SolutionType {
    hrvz() + vqtr()
}
fn flzm() -> SolutionType {
    shlq() * ngtq()
}
fn ffnp() -> SolutionType {
    11
}
fn cblv() -> SolutionType {
    ccww() * lgsh()
}
fn hdmg() -> SolutionType {
    3
}
fn mvsn() -> SolutionType {
    2
}
fn hcfh() -> SolutionType {
    dzwz() * zwrj()
}
fn qrvj() -> SolutionType {
    zgnm() + rzsf()
}
fn hjln() -> SolutionType {
    bjhz() * grsp()
}
fn lfnh() -> SolutionType {
    wqtw() + cdfp()
}
fn nrlg() -> SolutionType {
    cpcf() * wcrq()
}
fn htfd() -> SolutionType {
    5
}
fn ftcp() -> SolutionType {
    3
}
fn rdjh() -> SolutionType {
    hnph() + tlzg()
}
fn rngp() -> SolutionType {
    4
}
fn pmpj() -> SolutionType {
    lfhq() + srnd()
}
fn tjbj() -> SolutionType {
    lngh() + pswf()
}
fn ffhz() -> SolutionType {
    7
}
fn fvvp() -> SolutionType {
    fvld() + mmsr()
}
fn dcvl() -> SolutionType {
    gzrs() + srzh()
}
fn qndn() -> SolutionType {
    2
}
fn vhzm() -> SolutionType {
    gvnr() * dqtn()
}
fn gmjh() -> SolutionType {
    dbnp() + zmtj()
}
fn dwwh() -> SolutionType {
    14
}
fn fbzw() -> SolutionType {
    7
}
fn mdlw() -> SolutionType {
    4
}
fn smsc() -> SolutionType {
    rnpw() * pjhg()
}
fn qbtg() -> SolutionType {
    dphb() * bgzv()
}
fn mwhz() -> SolutionType {
    3
}
fn ltbw() -> SolutionType {
    tbgt() * hvwt()
}
fn fczq() -> SolutionType {
    4
}
fn bcmf() -> SolutionType {
    3
}
fn fsrp() -> SolutionType {
    dmwr() + qbqz()
}
fn zcgf() -> SolutionType {
    clgq() * nhch()
}
fn sqtg() -> SolutionType {
    pzms() + ffhz()
}
fn qtjv() -> SolutionType {
    cbmh() + wqmn()
}
fn llvf() -> SolutionType {
    mzsw() * tfgc()
}
fn qnwr() -> SolutionType {
    szqq() + gbtm()
}
fn bnqv() -> SolutionType {
    fccf() * tdbz()
}
fn rsgn() -> SolutionType {
    pnfz() - bfjm()
}
fn mvzs() -> SolutionType {
    vtnj() + flzm()
}
fn dgrd() -> SolutionType {
    jlwd() * fvdc()
}
fn fqvb() -> SolutionType {
    5
}
fn qvsg() -> SolutionType {
    2
}
fn lzvr() -> SolutionType {
    4
}
fn grnz() -> SolutionType {
    rmzm() * wztc()
}
fn srzh() -> SolutionType {
    5
}
fn jlzj() -> SolutionType {
    cvbm() + jdlb()
}
fn hzfw() -> SolutionType {
    gcdw() * rfsb()
}
fn ldfw() -> SolutionType {
    jgbd() * ztgg()
}
fn fhfs() -> SolutionType {
    dvzd() + srmj()
}
fn wptw() -> SolutionType {
    fpmd() + mmgl()
}
fn qdrw() -> SolutionType {
    1
}
fn wscd() -> SolutionType {
    nbfp() * bbqm()
}
fn mmrp() -> SolutionType {
    4
}
fn jpmm() -> SolutionType {
    2
}
fn rqhj() -> SolutionType {
    2
}
fn lhrr() -> SolutionType {
    pczh() + cfwm()
}
fn fvfz() -> SolutionType {
    8
}
fn mgwv() -> SolutionType {
    3
}
fn nnhg() -> SolutionType {
    2
}
fn mwsf() -> SolutionType {
    phlv() - fbzp()
}
fn dgdm() -> SolutionType {
    csfb() * zzsc()
}
fn zcld() -> SolutionType {
    lpwg() * ctqm()
}
fn zcsm() -> SolutionType {
    2
}
fn vbbp() -> SolutionType {
    bvsz() + qwtq()
}
fn bsmt() -> SolutionType {
    2
}
fn htcl() -> SolutionType {
    zvcw() + zbct()
}
fn msth() -> SolutionType {
    cppz() * fcgn()
}
fn ptrt() -> SolutionType {
    2
}
fn hhhz() -> SolutionType {
    zzjv() + mpln()
}
fn gqzl() -> SolutionType {
    mbgh() + thrw()
}
fn tjcw() -> SolutionType {
    gwbj() * ctjb()
}
fn bttm() -> SolutionType {
    1
}
fn jqqz() -> SolutionType {
    8
}
fn vgwv() -> SolutionType {
    mthn() * tcmb()
}
fn nzpv() -> SolutionType {
    dwhf() + fpmz()
}
fn fflf() -> SolutionType {
    jrlh() * prdz()
}
fn glml() -> SolutionType {
    mnjg() + ntpp()
}
fn vdrj() -> SolutionType {
    3
}
fn hbqz() -> SolutionType {
    zgvw() * llvf()
}
fn pbnq() -> SolutionType {
    zgnh() * lzvr()
}
fn grzf() -> SolutionType {
    wblz() + bjrz()
}
fn hlnd() -> SolutionType {
    4
}
fn ctcj() -> SolutionType {
    14
}
fn rnhn() -> SolutionType {
    3
}
fn rpcz() -> SolutionType {
    2
}
fn pcpq() -> SolutionType {
    dwnn() * ncqp()
}
fn mwhb() -> SolutionType {
    smqn() + jqqz()
}
fn lghh() -> SolutionType {
    3
}
fn ntcc() -> SolutionType {
    vqbp() * hcpm()
}
fn nfjf() -> SolutionType {
    3
}
fn bjrj() -> SolutionType {
    llsf() * qwmp()
}
fn qhcz() -> SolutionType {
    2
}
fn mzvj() -> SolutionType {
    hzfw() / llqh()
}
fn jfzs() -> SolutionType {
    ncct() * lqsz()
}
fn fdsq() -> SolutionType {
    hmzh() * lnhj()
}
fn rqws() -> SolutionType {
    bjsm() + dsgb()
}
fn fvwz() -> SolutionType {
    3
}
fn gmfd() -> SolutionType {
    mfqg() * djjq()
}
fn qwmp() -> SolutionType {
    8
}
fn rrwd() -> SolutionType {
    5
}
fn vjgg() -> SolutionType {
    gblf() * bnnf()
}
fn smlb() -> SolutionType {
    qbtg() * sjtc()
}
fn jjmt() -> SolutionType {
    qlvl() + lrqp()
}
fn vrfj() -> SolutionType {
    bjhw() * vsdz()
}
fn qmwt() -> SolutionType {
    2
}
fn fmvb() -> SolutionType {
    wfpd() * grcs()
}
fn fbzp() -> SolutionType {
    vsjb() * cgdm()
}
fn mvvj() -> SolutionType {
    hpct() * cgvg()
}
fn lhfj() -> SolutionType {
    3
}
fn lmvz() -> SolutionType {
    3
}
fn csdd() -> SolutionType {
    twlb() + jcsf()
}
fn ffsp() -> SolutionType {
    3
}
fn rwzq() -> SolutionType {
    sbvw() + trhm()
}
fn wwrg() -> SolutionType {
    btqz() - fwfg()
}
fn vnzf() -> SolutionType {
    3
}
fn sdfb() -> SolutionType {
    3
}
fn vvzv() -> SolutionType {
    5
}
fn lzqq() -> SolutionType {
    nhtm() + hmbd()
}
fn mspf() -> SolutionType {
    13
}
fn qswz() -> SolutionType {
    bwnd() + spdg()
}
fn mvjg() -> SolutionType {
    4
}
fn wrnm() -> SolutionType {
    3
}
fn qmqj() -> SolutionType {
    vfch() * jqpc()
}
fn tzmw() -> SolutionType {
    1
}
fn dbgb() -> SolutionType {
    1
}
fn lhtz() -> SolutionType {
    14
}
fn rlmj() -> SolutionType {
    3
}
fn mnvs() -> SolutionType {
    dgvr() + fdsq()
}
fn rfqt() -> SolutionType {
    15
}
fn rbsn() -> SolutionType {
    lzvh() + mwjg()
}
fn tcdl() -> SolutionType {
    5
}
fn zbrc() -> SolutionType {
    qmqj() + htlp()
}
fn vldd() -> SolutionType {
    fmcd() * llbs()
}
fn sczr() -> SolutionType {
    mnmz() * whlf()
}
fn gfwh() -> SolutionType {
    jdqd() + bhhw()
}
fn mqnz() -> SolutionType {
    8
}
fn mvlb() -> SolutionType {
    1
}
fn wbnj() -> SolutionType {
    3
}
fn psvd() -> SolutionType {
    gprr() * qvgv()
}
fn cgvg() -> SolutionType {
    2
}
fn vhbj() -> SolutionType {
    4
}
fn djdj() -> SolutionType {
    2
}
fn nwmr() -> SolutionType {
    zgzb() + lhmg()
}
fn hznr() -> SolutionType {
    cscs() * ndhc()
}
fn hsgr() -> SolutionType {
    17
}
fn jnhd() -> SolutionType {
    8
}
fn hrsl() -> SolutionType {
    9
}
fn fsqw() -> SolutionType {
    ctdm() * ftrj()
}
fn jcgs() -> SolutionType {
    4
}
fn rddp() -> SolutionType {
    mcgj() + jtqj()
}
fn sbsn() -> SolutionType {
    3
}
fn jsqd() -> SolutionType {
    lpms() * rgvf()
}
fn bnnj() -> SolutionType {
    zhll() + gcdn()
}
fn zvcw() -> SolutionType {
    10
}
fn vnct() -> SolutionType {
    dthl() + qmnt()
}
fn dsmw() -> SolutionType {
    ztht() + bqlh()
}
fn sznf() -> SolutionType {
    2
}
fn fvbg() -> SolutionType {
    2
}
fn plwr() -> SolutionType {
    5
}
fn cmcj() -> SolutionType {
    psbz() * gzrp()
}
fn hbtr() -> SolutionType {
    ldjc() * gjjw()
}

static HUMN: AtomicI64 = AtomicI64::new(2906);

#[aoc(day21, part1)]
pub fn solve_part1(_data: &[InputType]) -> SolutionType {
    root()
}

#[aoc(day21, part2)]
pub fn solve_part2(_data: &[InputType]) -> SolutionType {
    // Ugly, data dependant solution. But it works for me :)
    HUMN.store(0, Ordering::SeqCst);
    let z = dbg!(zmvq());
    loop {
        let r = dbcq();
        if r == z {
            break;
        }
        let i = (r - z) / 19;
        HUMN.fetch_add(i, Ordering::SeqCst);
    }
    HUMN.load(Ordering::SeqCst)
}
