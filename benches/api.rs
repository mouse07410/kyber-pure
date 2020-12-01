use criterion::{criterion_group, criterion_main, Criterion};
use pqc_kyber::*;

// First set in KAT_2400
const PK_HEX: &str = "A0B71F67C6CEC0D35686D513423432E512AC4044557E868A624800109A3355F98F151444E2852E27EA6EDB1992CAD3973C3A6FF79A5A049A259EB5415AA2A262456EC9495BBB5200D8D3163A5B10226292ECA01021389DA37881E276306550C6EFB6440EC51A2F7348349B851CD4AA0175A0550213C4791D91011220824B2B61650813ADFD2CB10538BFAB0A726F81129ED2C0F06A16B701090BF048C5A40126D572FCD47AA1218FB01547D150792D2316CB320D5144BA3508A1EBBB5AC1C22913E8295FAB59BF5837A778CF28227E07E1032DAB7D0E09A15F134148C12009DA536B22CC62474E69CC1554C0814D6CA0B722594383A9D0A2C77FD365A5544295FBB973F91EA56490D6CA6876497B98B3CB12417C257B6D0F7183DBB69E33075BEB0117B6914C69BA38349422F2F43364822A2570952DD5077B90755F1574115B8E221427585961913A9BFA0502B5D79AB7811744E6563C5B62C5CC4E93239A0A8CC60FE848F84A95F5902599B54A066293A2021DA196766C17C7E863AF790C270B216A25138DDA0C8126E09377879859DB358F9B82B7C8A6792ACEE92A4CBDE3CEDD4500ACBC555C288EFF9795265B9005351C52E2653554ABAAF872DF95CA7F795903F0B0A182B18AEB0475B29F6E3ABF4C2250FE7B842A73655016A8FC729F390507ACA936825A98B3A32E6B2554CE9528941A3BB8C90996008D74FBCD020A02E706A6DE7B02AF404C10DB00FAEC02D3EAA6D9561A1565A7B05C6366D09DA7A537F20C7B2859A83E029E13A9BD289157C5B74C84EAA307753D431202A3D9B6162218BEC5346945BFEF55B624C5C6E373359BB1C479952BBABA4D6555C276573E5152B553901999F69402D150BEF79D74FB2953018FF48666746ACE607814A1FA33195720F83878D3B575C725744A72070DD044018042DA25714D173090323A51E6C063D203881380912761FC3410839095F26C0E687A00705495E171B57151ACE0498E30F14CA9B02F6E40831854C2E0AB1ECD0C21D8E4C7E669CD728230B9D11F72C266E34466F9C0159EF424F8F31D95A57BA0E210543C10C6503FB5C63ED23AA36CD6A6F378261B0B1E79509D8BEB36AA263DC91545E53369DF26837F394C56777C95B648BD1A72921ABF49563F99CB9D98EAB5C66666F6B16F74022481FA214E617698D3BBD13CB308713FDCC7CFD397B9CA39AFF4C744D5715D58966F2CF9707015C8F3543ED286A3D8D5CBF64ACEDFC02971A91072C69D2EF49829F1037F050C5B92229856CB12B456CC095282A62687EA38C9778AEA491DFF069711FBBE05E8CD9BF44A8E712619573E12EAA7B23829DC6726BFE33DA136B81E153251508F6285BA15B2C1237677FE5B14B4E33F98C326BC58B9D8E075A25B94C8A23233029DCC786B135C56164BA3D160CBCEA854B7971F9CD73A383AAC050A302AD83B3E3AB90246AD160A321D330ACDEC7CA6643D7EC01F91691F16325BDF396950B88DAFE369C654B852055C970362C61380460757C65890F4E59222E4A4060B26C0EBC10197590DE3C8F0955D654B371CCB90ACA371B294476C16A4596A1DE8309E2A3612C69B7125310501E0C049B87440D9A6D0ECB999C9A0942AA340F60365EAFD465FC64A0C5F8F3F9003489415899D59A543D8208C54A3166529B53922";
const SK_HEX: &str = "07638FB69868F3D320E5862BD96933FEB311B362093C9B5D50170BCED43F1B536D9A204BB1F22695950BA1F2A9E8EB828B284488760B3FC84FABA04275D5628E39C5B2471374283C503299C0AB49B66B8BBB56A4186624F919A2BA59BB08D8551880C2BEFC4F87F25F59AB587A79C327D792D54C974A69262FF8A78938289E9A87B688B083E0595FE218B6BB1505941CE2E81A5A64C5AAC60417256985349EE47A52420A5F97477B7236AC76BC70E8288729287EE3E34A3DBC3683C0B7B10029FC203418537E7466BA6385A8FF301EE12708F82AAA1E380FC7A88F8F205AB7E88D7E95952A55BA20D09B79A47141D62BF6EB7DD307B08ECA13A5BC5F6B68581C6865B27BBCDDAB142F4B2CBFF488C8A22705FAA98A2B9EEA3530C76662335CC7EA3A00777725EBCCCD2A4636B2D9122FF3AB77123CE0883C1911115E50C9E8A94194E48DD0D09CFFB3ADCD2C1E92430903D07ADBF00532031575AA7F9E7B5A1F3362DEC936D4043C05F2476C07578BC9CBAF2AB4E382727AD41686A96B2548820BB03B32F11B2811AD62F489E951632ABA0D1DF89680CC8A8B53B481D92A68D70B4EA1C3A6A561C0692882B5CA8CC942A8D495AFCB06DE89498FB935B775908FE7A03E324D54CC19D4E1AABD3593B38B19EE1388FE492B43127E5A504253786A0D69AD32601C28E2C88504A5BA599706023A61363E17C6B9BB59BDC697452CD059451983D738CA3FD034E3F5988854CA05031DB09611498988197C6B30D258DFE26265541C89A4B31D6864E9389B03CB74F7EC4323FB9421A4B9790A26D17B0398A26767350909F84D57B6694DF830664CA8B3C3C03ED2AE67B89006868A68527CCD666459AB7F056671000C6164D3A7F266A14D97CBD7004D6C92CACA770B844A4FA9B182E7B18CA885082AC5646FCB4A14E1685FEB0C9CE3372AB95365C04FD83084F80A23FF10A05BF15F7FA5ACC6C0CB462C33CA524FA6B8BB359043BA68609EAA2536E81D08463B19653B5435BA946C9ADDEB202B04B031CC960DCC12E4518D428B32B257A4FC7313D3A7980D80082E934F9D95C32B0A0191A23604384DD9E079BBBAA266D14C3F756B9F2133107433A4E83FA7187282A809203A4FAF841851833D121AC383843A5E55BC2381425E16C7DB4CC9AB5C1B0D91A47E2B8DE0E582C86B6B0D907BB360B97F40AB5D038F6B75C814B27D9B968D419832BC8C2BEE605EF6E5059D33100D90485D378450014221736C07407CAC260408AA64926619788B8601C2A752D1A6CBF820D7C7A04716203225B3895B9342D147A8185CFC1BB65BA06B4142339903C0AC4651385B45D98A8B19D28CD6BAB088787F7EE1B12461766B43CBCCB96434427D93C065550688F6948ED1B5475A425F1B85209D061C08B56C1CC069F6C0A7C6F29358CAB911087732A649D27C9B98F9A48879387D9B00C25959A71654D6F6A946164513E47A75D005986C2363C09F6B537ECA78B9303A5FA457608A586A653A347DB04DFCC19175B3A301172536062A658A95277570C8852CA8973F4AE123A334047DD711C8927A634A03388A527B034BF7A8170FA702C1F7C23EC32D18A2374890BE9C787A9409C82D192C4BB705A2F996CE405DA0B71F67C6CEC0D35686D513423432E512AC4044557E868A624800109A3355F98F151444E2852E27EA6EDB1992CAD3973C3A6FF79A5A049A259EB5415AA2A262456EC9495BBB5200D8D3163A5B10226292ECA01021389DA37881E276306550C6EFB6440EC51A2F7348349B851CD4AA0175A0550213C4791D91011220824B2B61650813ADFD2CB10538BFAB0A726F81129ED2C0F06A16B701090BF048C5A40126D572FCD47AA1218FB01547D150792D2316CB320D5144BA3508A1EBBB5AC1C22913E8295FAB59BF5837A778CF28227E07E1032DAB7D0E09A15F134148C12009DA536B22CC62474E69CC1554C0814D6CA0B722594383A9D0A2C77FD365A5544295FBB973F91EA56490D6CA6876497B98B3CB12417C257B6D0F7183DBB69E33075BEB0117B6914C69BA38349422F2F43364822A2570952DD5077B90755F1574115B8E221427585961913A9BFA0502B5D79AB7811744E6563C5B62C5CC4E93239A0A8CC60FE848F84A95F5902599B54A066293A2021DA196766C17C7E863AF790C270B216A25138DDA0C8126E09377879859DB358F9B82B7C8A6792ACEE92A4CBDE3CEDD4500ACBC555C288EFF9795265B9005351C52E2653554ABAAF872DF95CA7F795903F0B0A182B18AEB0475B29F6E3ABF4C2250FE7B842A73655016A8FC729F390507ACA936825A98B3A32E6B2554CE9528941A3BB8C90996008D74FBCD020A02E706A6DE7B02AF404C10DB00FAEC02D3EAA6D9561A1565A7B05C6366D09DA7A537F20C7B2859A83E029E13A9BD289157C5B74C84EAA307753D431202A3D9B6162218BEC5346945BFEF55B624C5C6E373359BB1C479952BBABA4D6555C276573E5152B553901999F69402D150BEF79D74FB2953018FF48666746ACE607814A1FA33195720F83878D3B575C725744A72070DD044018042DA25714D173090323A51E6C063D203881380912761FC3410839095F26C0E687A00705495E171B57151ACE0498E30F14CA9B02F6E40831854C2E0AB1ECD0C21D8E4C7E669CD728230B9D11F72C266E34466F9C0159EF424F8F31D95A57BA0E210543C10C6503FB5C63ED23AA36CD6A6F378261B0B1E79509D8BEB36AA263DC91545E53369DF26837F394C56777C95B648BD1A72921ABF49563F99CB9D98EAB5C66666F6B16F74022481FA214E617698D3BBD13CB308713FDCC7CFD397B9CA39AFF4C744D5715D58966F2CF9707015C8F3543ED286A3D8D5CBF64ACEDFC02971A91072C69D2EF49829F1037F050C5B92229856CB12B456CC095282A62687EA38C9778AEA491DFF069711FBBE05E8CD9BF44A8E712619573E12EAA7B23829DC6726BFE33DA136B81E153251508F6285BA15B2C1237677FE5B14B4E33F98C326BC58B9D8E075A25B94C8A23233029DCC786B135C56164BA3D160CBCEA854B7971F9CD73A383AAC050A302AD83B3E3AB90246AD160A321D330ACDEC7CA6643D7EC01F91691F16325BDF396950B88DAFE369C654B852055C970362C61380460757C65890F4E59222E4A4060B26C0EBC10197590DE3C8F0955D654B371CCB90ACA371B294476C16A4596A1DE8309E2A3612C69B7125310501E0C049B87440D9A6D0ECB999C9A0942AA340F60365EAFD465FC64A0C5F8F3F9003489415899D59A543D8208C54A3166529B53922DEE4ABA000389581717D36F56F39AF7300B31D831A4D8C976128E09DEDE71A5A8626ED79D451140800E03B59B956F8210E556067407D13DC90FA9E8B872BFB8F";
const BAD_SK: &str = "17638FB69868F3D320E5862BD96933FEB311B362093C9B5D50170BCED43F1B536D9A204BB1F22695950BA1F2A9E8EB828B284488760B3FC84FABA04275D5628E39C5B2471374283C503299C0AB49B66B8BBB56A4186624F919A2BA59BB08D8551880C2BEFC4F87F25F59AB587A79C327D792D54C974A69262FF8A78938289E9A87B688B083E0595FE218B6BB1505941CE2E81A5A64C5AAC60417256985349EE47A52420A5F97477B7236AC76BC70E8288729287EE3E34A3DBC3683C0B7B10029FC203418537E7466BA6385A8FF301EE12708F82AAA1E380FC7A88F8F205AB7E88D7E95952A55BA20D09B79A47141D62BF6EB7DD307B08ECA13A5BC5F6B68581C6865B27BBCDDAB142F4B2CBFF488C8A22705FAA98A2B9EEA3530C76662335CC7EA3A00777725EBCCCD2A4636B2D9122FF3AB77123CE0883C1911115E50C9E8A94194E48DD0D09CFFB3ADCD2C1E92430903D07ADBF00532031575AA7F9E7B5A1F3362DEC936D4043C05F2476C07578BC9CBAF2AB4E382727AD41686A96B2548820BB03B32F11B2811AD62F489E951632ABA0D1DF89680CC8A8B53B481D92A68D70B4EA1C3A6A561C0692882B5CA8CC942A8D495AFCB06DE89498FB935B775908FE7A03E324D54CC19D4E1AABD3593B38B19EE1388FE492B43127E5A504253786A0D69AD32601C28E2C88504A5BA599706023A61363E17C6B9BB59BDC697452CD059451983D738CA3FD034E3F5988854CA05031DB09611498988197C6B30D258DFE26265541C89A4B31D6864E9389B03CB74F7EC4323FB9421A4B9790A26D17B0398A26767350909F84D57B6694DF830664CA8B3C3C03ED2AE67B89006868A68527CCD666459AB7F056671000C6164D3A7F266A14D97CBD7004D6C92CACA770B844A4FA9B182E7B18CA885082AC5646FCB4A14E1685FEB0C9CE3372AB95365C04FD83084F80A23FF10A05BF15F7FA5ACC6C0CB462C33CA524FA6B8BB359043BA68609EAA2536E81D08463B19653B5435BA946C9ADDEB202B04B031CC960DCC12E4518D428B32B257A4FC7313D3A7980D80082E934F9D95C32B0A0191A23604384DD9E079BBBAA266D14C3F756B9F2133107433A4E83FA7187282A809203A4FAF841851833D121AC383843A5E55BC2381425E16C7DB4CC9AB5C1B0D91A47E2B8DE0E582C86B6B0D907BB360B97F40AB5D038F6B75C814B27D9B968D419832BC8C2BEE605EF6E5059D33100D90485D378450014221736C07407CAC260408AA64926619788B8601C2A752D1A6CBF820D7C7A04716203225B3895B9342D147A8185CFC1BB65BA06B4142339903C0AC4651385B45D98A8B19D28CD6BAB088787F7EE1B12461766B43CBCCB96434427D93C065550688F6948ED1B5475A425F1B85209D061C08B56C1CC069F6C0A7C6F29358CAB911087732A649D27C9B98F9A48879387D9B00C25959A71654D6F6A946164513E47A75D005986C2363C09F6B537ECA78B9303A5FA457608A586A653A347DB04DFCC19175B3A301172536062A658A95277570C8852CA8973F4AE123A334047DD711C8927A634A03388A527B034BF7A8170FA702C1F7C23EC32D18A2374890BE9C787A9409C82D192C4BB705A2F996CE405DA0B71F67C6CEC0D35686D513423432E512AC4044557E868A624800109A3355F98F151444E2852E27EA6EDB1992CAD3973C3A6FF79A5A049A259EB5415AA2A262456EC9495BBB5200D8D3163A5B10226292ECA01021389DA37881E276306550C6EFB6440EC51A2F7348349B851CD4AA0175A0550213C4791D91011220824B2B61650813ADFD2CB10538BFAB0A726F81129ED2C0F06A16B701090BF048C5A40126D572FCD47AA1218FB01547D150792D2316CB320D5144BA3508A1EBBB5AC1C22913E8295FAB59BF5837A778CF28227E07E1032DAB7D0E09A15F134148C12009DA536B22CC62474E69CC1554C0814D6CA0B722594383A9D0A2C77FD365A5544295FBB973F91EA56490D6CA6876497B98B3CB12417C257B6D0F7183DBB69E33075BEB0117B6914C69BA38349422F2F43364822A2570952DD5077B90755F1574115B8E221427585961913A9BFA0502B5D79AB7811744E6563C5B62C5CC4E93239A0A8CC60FE848F84A95F5902599B54A066293A2021DA196766C17C7E863AF790C270B216A25138DDA0C8126E09377879859DB358F9B82B7C8A6792ACEE92A4CBDE3CEDD4500ACBC555C288EFF9795265B9005351C52E2653554ABAAF872DF95CA7F795903F0B0A182B18AEB0475B29F6E3ABF4C2250FE7B842A73655016A8FC729F390507ACA936825A98B3A32E6B2554CE9528941A3BB8C90996008D74FBCD020A02E706A6DE7B02AF404C10DB00FAEC02D3EAA6D9561A1565A7B05C6366D09DA7A537F20C7B2859A83E029E13A9BD289157C5B74C84EAA307753D431202A3D9B6162218BEC5346945BFEF55B624C5C6E373359BB1C479952BBABA4D6555C276573E5152B553901999F69402D150BEF79D74FB2953018FF48666746ACE607814A1FA33195720F83878D3B575C725744A72070DD044018042DA25714D173090323A51E6C063D203881380912761FC3410839095F26C0E687A00705495E171B57151ACE0498E30F14CA9B02F6E40831854C2E0AB1ECD0C21D8E4C7E669CD728230B9D11F72C266E34466F9C0159EF424F8F31D95A57BA0E210543C10C6503FB5C63ED23AA36CD6A6F378261B0B1E79509D8BEB36AA263DC91545E53369DF26837F394C56777C95B648BD1A72921ABF49563F99CB9D98EAB5C66666F6B16F74022481FA214E617698D3BBD13CB308713FDCC7CFD397B9CA39AFF4C744D5715D58966F2CF9707015C8F3543ED286A3D8D5CBF64ACEDFC02971A91072C69D2EF49829F1037F050C5B92229856CB12B456CC095282A62687EA38C9778AEA491DFF069711FBBE05E8CD9BF44A8E712619573E12EAA7B23829DC6726BFE33DA136B81E153251508F6285BA15B2C1237677FE5B14B4E33F98C326BC58B9D8E075A25B94C8A23233029DCC786B135C56164BA3D160CBCEA854B7971F9CD73A383AAC050A302AD83B3E3AB90246AD160A321D330ACDEC7CA6643D7EC01F91691F16325BDF396950B88DAFE369C654B852055C970362C61380460757C65890F4E59222E4A4060B26C0EBC10197590DE3C8F0955D654B371CCB90ACA371B294476C16A4596A1DE8309E2A3612C69B7125310501E0C049B87440D9A6D0ECB999C9A0942AA340F60365EAFD465FC64A0C5F8F3F9003489415899D59A543D8208C54A3166529B53922DEE4ABA000389581717D36F56F39AF7300B31D831A4D8C976128E09DEDE71A5A8626ED79D451140800E03B59B956F8210E556067407D13DC90FA9E8B872BFB8F";
const CT_HEX: &str = "EADD5ADA14DA57F0AEF3505F1CAA6485D4238D999A3EF4B0A59A1CDBE0A27E478547A3A99D2AB09AC7D7C8F5AE3D6432045CBA3FA778345892542BD81C05BEFCD2E5CC9A579BEFB7C58D02FB94F33392FE17F4EBA2CB510EC74CC9D1D8A87C1066A4869A3983E664BFE9DEA5AE4FDF310C8F59815A678FA325F369AF84FFEBC1D150431FE3BD2734F636CF658E6C1A6A6E2CBE071F9A7C26119AD105098EDA622CAB8E176762109877D9AE9D6729D44A58E707D6B8AD6E696A33C672DA9D08DA2A7F9E3BF02218238722A46B31D49DAFF9AF00A6363C3A423B2E873DEFDDBCD969B75A81053D9A97C06DE2BFE3D0CFD3D3C77983B18DBDE23C0728604A71435AD40DF1579096DDBE02E4612210CAA034DCEFB8B4D7B5E6D2EBA37A79FB61F34B5AF7D9B27B13E4936222411249B7FBB69E73461DAF4AA6F3E2C73944F10CE67C86FED260BDA7B40DB39B1DE3C7D8F09A77F3C84BC62931D228B24A574AC3F4EB745CFF7E031A3FB2A08595C15370A3C82DB7D9F41BB1D8ECC429CFA3A65833016AB6EA60C9390CFA1B65CCEAE550940795386ED24133FBAE8B3017502AF3CFE951D781D36CFEFF85BFDF5AF040BE4065681B3B0A63C2747F0808CF3DA725169DDED1003DA6CD5DE4CB041942938D0A7F8802D48F2E3C6EEB45CD90AF6FC9F4507E9F8380AC33CACA7751487F65500441D920B94880A497D01C0802BB08D74C5D4C6BF2D865EE5822B3375C755D1A5E3D3244C320510A1E30357702CD4252072CF86437F7A9DE5561C7E59B94B9584100131AC399F4C1EB19FB4BDF65E62785E97C194B8764CCF32FD05D804C2E439DDA2A109274FBFFA81A837C51B26D154F974B882A5B174B308FC48768D222922532B183ABDF6FBB0BC7492766974D321EE6FB7C5F7B3EEA2378DC6D6BB48019250B8D8D8DEDB522421AEEDB318676982A80E7961EC40E6D7F3339694255BAFF51BE3A7EA7D8793A109BE3AE4423BF082E206A573B4F0F93FC16DDE81BD5DC583F528C08A0A9AB8E6CD524E297C9CF0F43C344913830ECB16F91441477BA782EDD4E73E732979D3A664EB99EA5D24B6C84AA69F377CB0CAD5AE4E641E38B197A0994D58B2387E91760E9B6FEBCB445CF85BBA24A94CDA75E338674428249FE6DE4692601D1EAE0EA021D9BC8077BE8665D0737748FA30FCF80F7E482584674F633A5006A538267627FD91854E0871268A6B0B05DD51495135DEFB9376E9B841B64E5DBF43CE6C74BCF3AE1FC427E810B7CBF6957DBF904690E87842543897DE78F13D08D92EBD27FB2CFCC0C765430589057B16B15F207CA1E6F08D52616DD57AD43EFEA6FDDAAEA18D33731FAC7ECAAE950E1DF3C5A4E6FCB223DF5E86B487FD7092D0822EFFAEC82C4BEC10C600FDB90E77482911B1595277738841409D0F8F113191D47F5E56C115A05DEA759AA6FB1D047F9FCA4ED519EA5D21FE3BA5B9434FEA1283DFAD63D01589B0EB61F244351D03341DCD4DF62265AFCAEC6676A877D5CACB359EBB5319610DD447DA97E950B0C";
const _SS_HEX: &str = "ED20140C05D78B15F2E412671A84154217FD77619A2C522D3C3CB688CB34C68B";

// Benchmarking key generation
fn keypair_bench(c: &mut Criterion) {
  let mut rng = rand::thread_rng();
  c.bench_function(
    "Keypair Generation", 
    |b| b.iter(
      || {
        let _keys = keypair(&mut rng);
      }
    )
  );
}

// Encapsulating a single public key
fn encap_bench(c: &mut Criterion) {
  let pk = crate::decode_hex(PK_HEX);
  let mut rng = rand::thread_rng();
  c.bench_function(
    "Encapsulate", 
    |b| b.iter(
      || {
        let _enc = encapsulate(&pk, &mut rng);
      }
    )
  );
}

// Decapsulating a single correct ciphertext
fn decap_bench(c: &mut Criterion) {
  let sk = decode_hex(SK_HEX);
  let ct = decode_hex(CT_HEX);
  c.bench_function(
    "Decapsulate", 
    |b| b.iter(
      || {
        let _dec = decapsulate(&ct, &sk);
      }
    )
  );
}

// Decapsulating a single incorrect ciphertext
fn decap_fail_bench(c: &mut Criterion) {
  let sk = decode_hex(BAD_SK);
  let ct = decode_hex(CT_HEX);
  c.bench_function(
    "Decapsulate Failure", 
    |b| b.iter(
      || {
        let _dec = decapsulate(&ct, &sk);
      }
    )
  );
}

criterion_group!(benches, keypair_bench, encap_bench, decap_bench, decap_fail_bench);
criterion_main!(benches);

// Decodes a hex string into a vector of bytes
pub fn decode_hex(s: &str) -> Vec<u8> {
  (0..s.len())
      .step_by(2)
      .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("Hex string decoding"))
      .collect::<Vec<u8>>()
}