"use strict";

// see: https://stackoverflow.com/questions/2008279/validate-a-javascript-function-name
const validVariableNameRegex = /^(?!(?:do|if|in|for|let|new|try|var|case|else|enum|eval|false|null|this|true|void|with|break|catch|class|const|super|throw|while|yield|delete|export|import|public|return|static|switch|typeof|default|extends|finally|package|private|continue|debugger|function|arguments|interface|protected|implements|instanceof)$)[$A-Z\_a-z\xaa\xb5\xba\xc0-\xd6\xd8-\xf6\xf8-\u02c1\u02c6-\u02d1\u02e0-\u02e4\u02ec\u02ee\u0370-\u0374\u0376\u0377\u037a-\u037d\u0386\u0388-\u038a\u038c\u038e-\u03a1\u03a3-\u03f5\u03f7-\u0481\u048a-\u0527\u0531-\u0556\u0559\u0561-\u0587\u05d0-\u05ea\u05f0-\u05f2\u0620-\u064a\u066e\u066f\u0671-\u06d3\u06d5\u06e5\u06e6\u06ee\u06ef\u06fa-\u06fc\u06ff\u0710\u0712-\u072f\u074d-\u07a5\u07b1\u07ca-\u07ea\u07f4\u07f5\u07fa\u0800-\u0815\u081a\u0824\u0828\u0840-\u0858\u08a0\u08a2-\u08ac\u0904-\u0939\u093d\u0950\u0958-\u0961\u0971-\u0977\u0979-\u097f\u0985-\u098c\u098f\u0990\u0993-\u09a8\u09aa-\u09b0\u09b2\u09b6-\u09b9\u09bd\u09ce\u09dc\u09dd\u09df-\u09e1\u09f0\u09f1\u0a05-\u0a0a\u0a0f\u0a10\u0a13-\u0a28\u0a2a-\u0a30\u0a32\u0a33\u0a35\u0a36\u0a38\u0a39\u0a59-\u0a5c\u0a5e\u0a72-\u0a74\u0a85-\u0a8d\u0a8f-\u0a91\u0a93-\u0aa8\u0aaa-\u0ab0\u0ab2\u0ab3\u0ab5-\u0ab9\u0abd\u0ad0\u0ae0\u0ae1\u0b05-\u0b0c\u0b0f\u0b10\u0b13-\u0b28\u0b2a-\u0b30\u0b32\u0b33\u0b35-\u0b39\u0b3d\u0b5c\u0b5d\u0b5f-\u0b61\u0b71\u0b83\u0b85-\u0b8a\u0b8e-\u0b90\u0b92-\u0b95\u0b99\u0b9a\u0b9c\u0b9e\u0b9f\u0ba3\u0ba4\u0ba8-\u0baa\u0bae-\u0bb9\u0bd0\u0c05-\u0c0c\u0c0e-\u0c10\u0c12-\u0c28\u0c2a-\u0c33\u0c35-\u0c39\u0c3d\u0c58\u0c59\u0c60\u0c61\u0c85-\u0c8c\u0c8e-\u0c90\u0c92-\u0ca8\u0caa-\u0cb3\u0cb5-\u0cb9\u0cbd\u0cde\u0ce0\u0ce1\u0cf1\u0cf2\u0d05-\u0d0c\u0d0e-\u0d10\u0d12-\u0d3a\u0d3d\u0d4e\u0d60\u0d61\u0d7a-\u0d7f\u0d85-\u0d96\u0d9a-\u0db1\u0db3-\u0dbb\u0dbd\u0dc0-\u0dc6\u0e01-\u0e30\u0e32\u0e33\u0e40-\u0e46\u0e81\u0e82\u0e84\u0e87\u0e88\u0e8a\u0e8d\u0e94-\u0e97\u0e99-\u0e9f\u0ea1-\u0ea3\u0ea5\u0ea7\u0eaa\u0eab\u0ead-\u0eb0\u0eb2\u0eb3\u0ebd\u0ec0-\u0ec4\u0ec6\u0edc-\u0edf\u0f00\u0f40-\u0f47\u0f49-\u0f6c\u0f88-\u0f8c\u1000-\u102a\u103f\u1050-\u1055\u105a-\u105d\u1061\u1065\u1066\u106e-\u1070\u1075-\u1081\u108e\u10a0-\u10c5\u10c7\u10cd\u10d0-\u10fa\u10fc-\u1248\u124a-\u124d\u1250-\u1256\u1258\u125a-\u125d\u1260-\u1288\u128a-\u128d\u1290-\u12b0\u12b2-\u12b5\u12b8-\u12be\u12c0\u12c2-\u12c5\u12c8-\u12d6\u12d8-\u1310\u1312-\u1315\u1318-\u135a\u1380-\u138f\u13a0-\u13f4\u1401-\u166c\u166f-\u167f\u1681-\u169a\u16a0-\u16ea\u16ee-\u16f0\u1700-\u170c\u170e-\u1711\u1720-\u1731\u1740-\u1751\u1760-\u176c\u176e-\u1770\u1780-\u17b3\u17d7\u17dc\u1820-\u1877\u1880-\u18a8\u18aa\u18b0-\u18f5\u1900-\u191c\u1950-\u196d\u1970-\u1974\u1980-\u19ab\u19c1-\u19c7\u1a00-\u1a16\u1a20-\u1a54\u1aa7\u1b05-\u1b33\u1b45-\u1b4b\u1b83-\u1ba0\u1bae\u1baf\u1bba-\u1be5\u1c00-\u1c23\u1c4d-\u1c4f\u1c5a-\u1c7d\u1ce9-\u1cec\u1cee-\u1cf1\u1cf5\u1cf6\u1d00-\u1dbf\u1e00-\u1f15\u1f18-\u1f1d\u1f20-\u1f45\u1f48-\u1f4d\u1f50-\u1f57\u1f59\u1f5b\u1f5d\u1f5f-\u1f7d\u1f80-\u1fb4\u1fb6-\u1fbc\u1fbe\u1fc2-\u1fc4\u1fc6-\u1fcc\u1fd0-\u1fd3\u1fd6-\u1fdb\u1fe0-\u1fec\u1ff2-\u1ff4\u1ff6-\u1ffc\u2071\u207f\u2090-\u209c\u2102\u2107\u210a-\u2113\u2115\u2119-\u211d\u2124\u2126\u2128\u212a-\u212d\u212f-\u2139\u213c-\u213f\u2145-\u2149\u214e\u2160-\u2188\u2c00-\u2c2e\u2c30-\u2c5e\u2c60-\u2ce4\u2ceb-\u2cee\u2cf2\u2cf3\u2d00-\u2d25\u2d27\u2d2d\u2d30-\u2d67\u2d6f\u2d80-\u2d96\u2da0-\u2da6\u2da8-\u2dae\u2db0-\u2db6\u2db8-\u2dbe\u2dc0-\u2dc6\u2dc8-\u2dce\u2dd0-\u2dd6\u2dd8-\u2dde\u2e2f\u3005-\u3007\u3021-\u3029\u3031-\u3035\u3038-\u303c\u3041-\u3096\u309d-\u309f\u30a1-\u30fa\u30fc-\u30ff\u3105-\u312d\u3131-\u318e\u31a0-\u31ba\u31f0-\u31ff\u3400-\u4db5\u4e00-\u9fcc\ua000-\ua48c\ua4d0-\ua4fd\ua500-\ua60c\ua610-\ua61f\ua62a\ua62b\ua640-\ua66e\ua67f-\ua697\ua6a0-\ua6ef\ua717-\ua71f\ua722-\ua788\ua78b-\ua78e\ua790-\ua793\ua7a0-\ua7aa\ua7f8-\ua801\ua803-\ua805\ua807-\ua80a\ua80c-\ua822\ua840-\ua873\ua882-\ua8b3\ua8f2-\ua8f7\ua8fb\ua90a-\ua925\ua930-\ua946\ua960-\ua97c\ua984-\ua9b2\ua9cf\uaa00-\uaa28\uaa40-\uaa42\uaa44-\uaa4b\uaa60-\uaa76\uaa7a\uaa80-\uaaaf\uaab1\uaab5\uaab6\uaab9-\uaabd\uaac0\uaac2\uaadb-\uaadd\uaae0-\uaaea\uaaf2-\uaaf4\uab01-\uab06\uab09-\uab0e\uab11-\uab16\uab20-\uab26\uab28-\uab2e\uabc0-\uabe2\uac00-\ud7a3\ud7b0-\ud7c6\ud7cb-\ud7fb\uf900-\ufa6d\ufa70-\ufad9\ufb00-\ufb06\ufb13-\ufb17\ufb1d\ufb1f-\ufb28\ufb2a-\ufb36\ufb38-\ufb3c\ufb3e\ufb40\ufb41\ufb43\ufb44\ufb46-\ufbb1\ufbd3-\ufd3d\ufd50-\ufd8f\ufd92-\ufdc7\ufdf0-\ufdfb\ufe70-\ufe74\ufe76-\ufefc\uff21-\uff3a\uff41-\uff5a\uff66-\uffbe\uffc2-\uffc7\uffca-\uffcf\uffd2-\uffd7\uffda-\uffdc][$A-Z\_a-z\xaa\xb5\xba\xc0-\xd6\xd8-\xf6\xf8-\u02c1\u02c6-\u02d1\u02e0-\u02e4\u02ec\u02ee\u0370-\u0374\u0376\u0377\u037a-\u037d\u0386\u0388-\u038a\u038c\u038e-\u03a1\u03a3-\u03f5\u03f7-\u0481\u048a-\u0527\u0531-\u0556\u0559\u0561-\u0587\u05d0-\u05ea\u05f0-\u05f2\u0620-\u064a\u066e\u066f\u0671-\u06d3\u06d5\u06e5\u06e6\u06ee\u06ef\u06fa-\u06fc\u06ff\u0710\u0712-\u072f\u074d-\u07a5\u07b1\u07ca-\u07ea\u07f4\u07f5\u07fa\u0800-\u0815\u081a\u0824\u0828\u0840-\u0858\u08a0\u08a2-\u08ac\u0904-\u0939\u093d\u0950\u0958-\u0961\u0971-\u0977\u0979-\u097f\u0985-\u098c\u098f\u0990\u0993-\u09a8\u09aa-\u09b0\u09b2\u09b6-\u09b9\u09bd\u09ce\u09dc\u09dd\u09df-\u09e1\u09f0\u09f1\u0a05-\u0a0a\u0a0f\u0a10\u0a13-\u0a28\u0a2a-\u0a30\u0a32\u0a33\u0a35\u0a36\u0a38\u0a39\u0a59-\u0a5c\u0a5e\u0a72-\u0a74\u0a85-\u0a8d\u0a8f-\u0a91\u0a93-\u0aa8\u0aaa-\u0ab0\u0ab2\u0ab3\u0ab5-\u0ab9\u0abd\u0ad0\u0ae0\u0ae1\u0b05-\u0b0c\u0b0f\u0b10\u0b13-\u0b28\u0b2a-\u0b30\u0b32\u0b33\u0b35-\u0b39\u0b3d\u0b5c\u0b5d\u0b5f-\u0b61\u0b71\u0b83\u0b85-\u0b8a\u0b8e-\u0b90\u0b92-\u0b95\u0b99\u0b9a\u0b9c\u0b9e\u0b9f\u0ba3\u0ba4\u0ba8-\u0baa\u0bae-\u0bb9\u0bd0\u0c05-\u0c0c\u0c0e-\u0c10\u0c12-\u0c28\u0c2a-\u0c33\u0c35-\u0c39\u0c3d\u0c58\u0c59\u0c60\u0c61\u0c85-\u0c8c\u0c8e-\u0c90\u0c92-\u0ca8\u0caa-\u0cb3\u0cb5-\u0cb9\u0cbd\u0cde\u0ce0\u0ce1\u0cf1\u0cf2\u0d05-\u0d0c\u0d0e-\u0d10\u0d12-\u0d3a\u0d3d\u0d4e\u0d60\u0d61\u0d7a-\u0d7f\u0d85-\u0d96\u0d9a-\u0db1\u0db3-\u0dbb\u0dbd\u0dc0-\u0dc6\u0e01-\u0e30\u0e32\u0e33\u0e40-\u0e46\u0e81\u0e82\u0e84\u0e87\u0e88\u0e8a\u0e8d\u0e94-\u0e97\u0e99-\u0e9f\u0ea1-\u0ea3\u0ea5\u0ea7\u0eaa\u0eab\u0ead-\u0eb0\u0eb2\u0eb3\u0ebd\u0ec0-\u0ec4\u0ec6\u0edc-\u0edf\u0f00\u0f40-\u0f47\u0f49-\u0f6c\u0f88-\u0f8c\u1000-\u102a\u103f\u1050-\u1055\u105a-\u105d\u1061\u1065\u1066\u106e-\u1070\u1075-\u1081\u108e\u10a0-\u10c5\u10c7\u10cd\u10d0-\u10fa\u10fc-\u1248\u124a-\u124d\u1250-\u1256\u1258\u125a-\u125d\u1260-\u1288\u128a-\u128d\u1290-\u12b0\u12b2-\u12b5\u12b8-\u12be\u12c0\u12c2-\u12c5\u12c8-\u12d6\u12d8-\u1310\u1312-\u1315\u1318-\u135a\u1380-\u138f\u13a0-\u13f4\u1401-\u166c\u166f-\u167f\u1681-\u169a\u16a0-\u16ea\u16ee-\u16f0\u1700-\u170c\u170e-\u1711\u1720-\u1731\u1740-\u1751\u1760-\u176c\u176e-\u1770\u1780-\u17b3\u17d7\u17dc\u1820-\u1877\u1880-\u18a8\u18aa\u18b0-\u18f5\u1900-\u191c\u1950-\u196d\u1970-\u1974\u1980-\u19ab\u19c1-\u19c7\u1a00-\u1a16\u1a20-\u1a54\u1aa7\u1b05-\u1b33\u1b45-\u1b4b\u1b83-\u1ba0\u1bae\u1baf\u1bba-\u1be5\u1c00-\u1c23\u1c4d-\u1c4f\u1c5a-\u1c7d\u1ce9-\u1cec\u1cee-\u1cf1\u1cf5\u1cf6\u1d00-\u1dbf\u1e00-\u1f15\u1f18-\u1f1d\u1f20-\u1f45\u1f48-\u1f4d\u1f50-\u1f57\u1f59\u1f5b\u1f5d\u1f5f-\u1f7d\u1f80-\u1fb4\u1fb6-\u1fbc\u1fbe\u1fc2-\u1fc4\u1fc6-\u1fcc\u1fd0-\u1fd3\u1fd6-\u1fdb\u1fe0-\u1fec\u1ff2-\u1ff4\u1ff6-\u1ffc\u2071\u207f\u2090-\u209c\u2102\u2107\u210a-\u2113\u2115\u2119-\u211d\u2124\u2126\u2128\u212a-\u212d\u212f-\u2139\u213c-\u213f\u2145-\u2149\u214e\u2160-\u2188\u2c00-\u2c2e\u2c30-\u2c5e\u2c60-\u2ce4\u2ceb-\u2cee\u2cf2\u2cf3\u2d00-\u2d25\u2d27\u2d2d\u2d30-\u2d67\u2d6f\u2d80-\u2d96\u2da0-\u2da6\u2da8-\u2dae\u2db0-\u2db6\u2db8-\u2dbe\u2dc0-\u2dc6\u2dc8-\u2dce\u2dd0-\u2dd6\u2dd8-\u2dde\u2e2f\u3005-\u3007\u3021-\u3029\u3031-\u3035\u3038-\u303c\u3041-\u3096\u309d-\u309f\u30a1-\u30fa\u30fc-\u30ff\u3105-\u312d\u3131-\u318e\u31a0-\u31ba\u31f0-\u31ff\u3400-\u4db5\u4e00-\u9fcc\ua000-\ua48c\ua4d0-\ua4fd\ua500-\ua60c\ua610-\ua61f\ua62a\ua62b\ua640-\ua66e\ua67f-\ua697\ua6a0-\ua6ef\ua717-\ua71f\ua722-\ua788\ua78b-\ua78e\ua790-\ua793\ua7a0-\ua7aa\ua7f8-\ua801\ua803-\ua805\ua807-\ua80a\ua80c-\ua822\ua840-\ua873\ua882-\ua8b3\ua8f2-\ua8f7\ua8fb\ua90a-\ua925\ua930-\ua946\ua960-\ua97c\ua984-\ua9b2\ua9cf\uaa00-\uaa28\uaa40-\uaa42\uaa44-\uaa4b\uaa60-\uaa76\uaa7a\uaa80-\uaaaf\uaab1\uaab5\uaab6\uaab9-\uaabd\uaac0\uaac2\uaadb-\uaadd\uaae0-\uaaea\uaaf2-\uaaf4\uab01-\uab06\uab09-\uab0e\uab11-\uab16\uab20-\uab26\uab28-\uab2e\uabc0-\uabe2\uac00-\ud7a3\ud7b0-\ud7c6\ud7cb-\ud7fb\uf900-\ufa6d\ufa70-\ufad9\ufb00-\ufb06\ufb13-\ufb17\ufb1d\ufb1f-\ufb28\ufb2a-\ufb36\ufb38-\ufb3c\ufb3e\ufb40\ufb41\ufb43\ufb44\ufb46-\ufbb1\ufbd3-\ufd3d\ufd50-\ufd8f\ufd92-\ufdc7\ufdf0-\ufdfb\ufe70-\ufe74\ufe76-\ufefc\uff21-\uff3a\uff41-\uff5a\uff66-\uffbe\uffc2-\uffc7\uffca-\uffcf\uffd2-\uffd7\uffda-\uffdc0-9\u0300-\u036f\u0483-\u0487\u0591-\u05bd\u05bf\u05c1\u05c2\u05c4\u05c5\u05c7\u0610-\u061a\u064b-\u0669\u0670\u06d6-\u06dc\u06df-\u06e4\u06e7\u06e8\u06ea-\u06ed\u06f0-\u06f9\u0711\u0730-\u074a\u07a6-\u07b0\u07c0-\u07c9\u07eb-\u07f3\u0816-\u0819\u081b-\u0823\u0825-\u0827\u0829-\u082d\u0859-\u085b\u08e4-\u08fe\u0900-\u0903\u093a-\u093c\u093e-\u094f\u0951-\u0957\u0962\u0963\u0966-\u096f\u0981-\u0983\u09bc\u09be-\u09c4\u09c7\u09c8\u09cb-\u09cd\u09d7\u09e2\u09e3\u09e6-\u09ef\u0a01-\u0a03\u0a3c\u0a3e-\u0a42\u0a47\u0a48\u0a4b-\u0a4d\u0a51\u0a66-\u0a71\u0a75\u0a81-\u0a83\u0abc\u0abe-\u0ac5\u0ac7-\u0ac9\u0acb-\u0acd\u0ae2\u0ae3\u0ae6-\u0aef\u0b01-\u0b03\u0b3c\u0b3e-\u0b44\u0b47\u0b48\u0b4b-\u0b4d\u0b56\u0b57\u0b62\u0b63\u0b66-\u0b6f\u0b82\u0bbe-\u0bc2\u0bc6-\u0bc8\u0bca-\u0bcd\u0bd7\u0be6-\u0bef\u0c01-\u0c03\u0c3e-\u0c44\u0c46-\u0c48\u0c4a-\u0c4d\u0c55\u0c56\u0c62\u0c63\u0c66-\u0c6f\u0c82\u0c83\u0cbc\u0cbe-\u0cc4\u0cc6-\u0cc8\u0cca-\u0ccd\u0cd5\u0cd6\u0ce2\u0ce3\u0ce6-\u0cef\u0d02\u0d03\u0d3e-\u0d44\u0d46-\u0d48\u0d4a-\u0d4d\u0d57\u0d62\u0d63\u0d66-\u0d6f\u0d82\u0d83\u0dca\u0dcf-\u0dd4\u0dd6\u0dd8-\u0ddf\u0df2\u0df3\u0e31\u0e34-\u0e3a\u0e47-\u0e4e\u0e50-\u0e59\u0eb1\u0eb4-\u0eb9\u0ebb\u0ebc\u0ec8-\u0ecd\u0ed0-\u0ed9\u0f18\u0f19\u0f20-\u0f29\u0f35\u0f37\u0f39\u0f3e\u0f3f\u0f71-\u0f84\u0f86\u0f87\u0f8d-\u0f97\u0f99-\u0fbc\u0fc6\u102b-\u103e\u1040-\u1049\u1056-\u1059\u105e-\u1060\u1062-\u1064\u1067-\u106d\u1071-\u1074\u1082-\u108d\u108f-\u109d\u135d-\u135f\u1712-\u1714\u1732-\u1734\u1752\u1753\u1772\u1773\u17b4-\u17d3\u17dd\u17e0-\u17e9\u180b-\u180d\u1810-\u1819\u18a9\u1920-\u192b\u1930-\u193b\u1946-\u194f\u19b0-\u19c0\u19c8\u19c9\u19d0-\u19d9\u1a17-\u1a1b\u1a55-\u1a5e\u1a60-\u1a7c\u1a7f-\u1a89\u1a90-\u1a99\u1b00-\u1b04\u1b34-\u1b44\u1b50-\u1b59\u1b6b-\u1b73\u1b80-\u1b82\u1ba1-\u1bad\u1bb0-\u1bb9\u1be6-\u1bf3\u1c24-\u1c37\u1c40-\u1c49\u1c50-\u1c59\u1cd0-\u1cd2\u1cd4-\u1ce8\u1ced\u1cf2-\u1cf4\u1dc0-\u1de6\u1dfc-\u1dff\u200c\u200d\u203f\u2040\u2054\u20d0-\u20dc\u20e1\u20e5-\u20f0\u2cef-\u2cf1\u2d7f\u2de0-\u2dff\u302a-\u302f\u3099\u309a\ua620-\ua629\ua66f\ua674-\ua67d\ua69f\ua6f0\ua6f1\ua802\ua806\ua80b\ua823-\ua827\ua880\ua881\ua8b4-\ua8c4\ua8d0-\ua8d9\ua8e0-\ua8f1\ua900-\ua909\ua926-\ua92d\ua947-\ua953\ua980-\ua983\ua9b3-\ua9c0\ua9d0-\ua9d9\uaa29-\uaa36\uaa43\uaa4c\uaa4d\uaa50-\uaa59\uaa7b\uaab0\uaab2-\uaab4\uaab7\uaab8\uaabe\uaabf\uaac1\uaaeb-\uaaef\uaaf5\uaaf6\uabe3-\uabea\uabec\uabed\uabf0-\uabf9\ufb1e\ufe00-\ufe0f\ufe20-\ufe26\ufe33\ufe34\ufe4d-\ufe4f\uff10-\uff19\uff3f]*$/;

function findClosingParenthesis(str, pos = 0) {
    let depth = 1;
    for (let i = pos + 1; i < str.length; ++i) {
        if (str[i] === ')' && --depth === 0) {
            return i;
        } else if (str[i] === '(') {
            depth++;
        }
    }
    return -1;
}

class ToSerializable {
    toSerializable() {
        return {};
    }
}

/**
 * A base class for serializable classes
 */
class Clone extends ToSerializable {
    clone() {
        return new this.constructor(JSON.parse(JSON.stringify(this.toSerializable())));
    }
}

export class Symbol extends Clone {
    #name;
    #parameters;

    constructor({name, parameters = []}) {
        super();
        if (!name.length) {
            throw Error('symbol name must not be empty');
        }
        this.#name = name;
        this.#parameters = parameters;
    }

    get name() {
        return this.#name;
    }

    get parameters() {
        return this.#parameters;
    }

    equals(other) {
        return this.name === other.name && this.parameters.length === other.parameters.length;
    }

    toGenericString() {
        return `${this.name}(${this.parameters.map(_ => '_')})`;
    }

    toSerializable() {
        return { name: this.name, parameters: this.parameters };
    }

    toString(ignoreEmptyParameters = false) {
        if (ignoreEmptyParameters && !this.parameters.length) {
            return `${this.name}`;
        } else {
            return `${this.name}(${this.parameters})`;
        }
    }

    static fromString(str) {
        if (str.includes('(')) {
            const openIdx = str.indexOf('(');
            return new Symbol({
                name: str.slice(0, openIdx),
                parameters: str.slice(openIdx + 1, findClosingParenthesis(str, openIdx)).split(','),
            });
        } else {
            return new Symbol({name: str});
        }
    }
}

export class Module extends Clone {
    #name;
    #parameters;
    #isQuery;

    constructor({name, parameters = []}) {
        super();
        if (!name.length) {
            throw Error('module name must not be empty');
        }
        this.#name = name;
        this.#parameters = parameters;
        this.#isQuery = name[0] === '?';

        if (this.#isQuery) {
            throw Error("query modules are not supported yet");
        }
    }

    get name() {
        return this.#name;
    }

    get parameters() {
        return this.#parameters;
    }

    get isQuery() {
        return this.#isQuery;
    }

    toGenericString() {
        return `${this.name}(${this.parameters.map(_ => '_')})`;
    }

    toString(ignoreEmptyParameters = false) {
        if (ignoreEmptyParameters && !this.parameters.length) {
            return `${this.name}`;
        } else {
            return `${this.name}(${this.parameters})`;
        }
    }

    toSerializable() {
        return { name: this.#name, parameters: this.parameters };
    }

    static fromString(str) {
        const symbol = Symbol.fromString(str);
        return new Module({
            name: symbol.name,
            parameters: symbol.parameters.map(Number),
        });
    }
}

class LSystemState extends Clone {
    #axiom;
    parameters;

    constructor({axiom = [], parameters = {}}) {
        super();
        this.#axiom = axiom;
        this.parameters = parameters;
    }

    get axiom() {
        return this.#axiom;
    }

    set axiom(value) {
        this.#axiom = value;
    }

    toSerializable() {
        return {
            axiom: this.#axiom.map(m => m.toSerializable()),
            parameters: this.parameters,
        }
    }

    toString() {
        return this.#axiom.map(m => m.toString(true)).join('')
    }
}

class Operation {
    #probability;
    #func;

    constructor(probability, func) {
        this.#probability = probability;
        this.#func = func;
    }

    get probability() {
        return this.#probability;
    }

    apply(module, systemState) {
        return this.#func(Module.prototype.constructor, ...module.parameters, systemState.parameters);
    }
}

class Production {
    /**
     * The symbol this production operates on.
     */
    #symbol;

    /**
     * A list of symbols that have to precede the symbol.
     */
    #predecessors;

    /**
     * A list of symbols that have to succeed the symbol.
     */
    #successors;

    /**
     * A condition that has to be satisfied for this production to get applied.
     */
    #condition;

    /**
     * A list of productions and their probabilities to be applied, sorted by their probabilities.
     */
    #operations;

    constructor({symbol, predecessors, successors, condition = undefined, operations}) {
        this.#symbol = symbol;
        this.#predecessors = predecessors;
        this.#successors = successors;
        this.#condition = condition;
        this.#operations = operations;
    }

    #chooseOperation() {
        const probability = Math.random();
        for (const op of this.#operations) {
            if (probability <= op.probability) {
                return op;
            }
        }
    }

    apply(module, systemState) {
        return this.#chooseOperation().apply(module, systemState);
    }

    condition(module, systemState) {
        return this.#condition ? this.#condition(...module.parameters, systemState.parameters) : true;
    }

    rank() {
        return this.#predecessors.length + this.#successors.length + (this.#condition ? 1 : 0);
    }

    get predecessors() {
        return this.#predecessors;
    }

    get successors() {
        return this.#successors;
    }
}

export class LSystemParser {
    #probabilitySeparator;
    #modulePredecessorSeparator;
    #moduleSuccessorSeparator;
    #conditionSeparator;
    #operationSeparator;


    constructor({
        probabilitySeparator = ';',
        modulePredecessorSeparator = '<',
        moduleSuccessorSeparator = '>',
        conditionSeparator = ':',
        operationSeparator = '->'
    }) {
        this.#probabilitySeparator = probabilitySeparator;
        this.#modulePredecessorSeparator = modulePredecessorSeparator;
        this.#moduleSuccessorSeparator = moduleSuccessorSeparator;
        this.#conditionSeparator = conditionSeparator;
        this.#operationSeparator = operationSeparator;
    }

    parseLSystem({alphabet = undefined, parameters = {}, productions, axiom}) {
        // two modules with the same name but different numbers of args are allowed!
        // module names are arbitrary, but the following symbols are not allowed:
        //  ? ( ) < > : ; -
        // if no alphabet is given it is parsed from the axiom and the productions in the following way
        //  - symbols for which productions exist are first extracted from the productions
        //  -

        const productionSpecs = productions.map(p => this.#splitProductionDefinition(p))
            .reduce((symbolsToProdSpecs, p) => {
                const symbol = p.moduleSpecification.module.toGenericString();
                if (!symbolsToProdSpecs[symbol]) {
                    symbolsToProdSpecs[symbol] = [];
                }
                symbolsToProdSpecs[symbol].push(p);
                return symbolsToProdSpecs;
            }, {});

        const symbols = [];
        if (!alphabet) {
            // todo: parse alphabet from productions & axiom
            for (const key of Object.keys(productionSpecs)) {
                symbols.push(productionSpecs[key][0].moduleSpecification.module);
            }

        } else {
            symbols.push(...alphabet.map(Symbol.fromString))
        }

        // sort by most specific to the least specific
        symbols.sort((a, b) => {
            if (a.name === b.name) {
                return b.parameters.length - a.parameters.length;
            }
            return b.name.length - a.name.length;
        });

        // at this point the alphabet must be complete, so we can parse the axiom & production specs
        return new LSystem({
            symbols,
            parameters,
            axiom: LSystemParser.#parseAxiom(axiom, symbols),
            productions: this.#processProductionSpecs(productionSpecs, symbols, parameters),
        })
    }

    static #conditionEquals(a, b) {
        let condA = `${a.condition.conditionDefinition}`;
        let condB = `${b.condition.conditionDefinition}`;
        for (const i in b.parameters) {
            condA = condA.replace(b.parameters[i], a.parameters[i]);
            condB = condB.replace(b.parameters[i], a.parameters[i]);
        }
        return condA === condB;
    }

    #processProductionSpecs(productionSpecs, symbols, parameters) {
        return Object.keys(productionSpecs).reduce((productions, symbolName) => {
            const specs = productionSpecs[symbolName].map(({probability, moduleSpecification, condition, body}) => {
                return {
                    probability,
                    symbol: moduleSpecification.module,
                    predecessors: LSystemParser.#parseProductionPreOrSuccessors(moduleSpecification.predecessors, symbols),
                    successors: LSystemParser.#parseProductionPreOrSuccessors(moduleSpecification.successors, symbols),
                    condition: this.#parseProductionCondition(condition, moduleSpecification.module.parameters, parameters),
                    body: this.#parseProductionBody(body, moduleSpecification.module.parameters, parameters, symbols),
                };
            });

            const processed = new Set();
            for (const i in specs) {
                if (processed.has(i)) {
                    continue;
                }

                const spec = specs[i];
                const operations = [];
                for (let j = i; j < specs.length; ++j) {
                    const s = specs[j];
                    if (s.predecessors.length !== spec.predecessors.length ||
                        s.successors.length !== spec.successors.length ||
                        !LSystemParser.#conditionEquals(spec, s)
                    ) {
                        continue;
                    }
                    let candidate = true;
                    for (const l in s.predecessors) {
                        if (!s.predecessors[l].equals(spec.predecessors[l])) {
                            candidate = false;
                            break;
                        }
                    }
                    if (!candidate) {
                        continue;
                    }
                    for (const l in s.successors) {
                        if (!s.successors[l].equals(spec.successors[l])) {
                            candidate = false;
                            break;
                        }
                    }
                    if (!candidate) {
                        continue;
                    }
                    operations.push(s);
                    processed.add(j);
                }

                const numNoProbability = operations.reduce((sum, c) => sum + (c.probability ? 0 : 1), 0);
                const sumProbability = operations.reduce((sum, c) => sum + (c.probability || 0), 0);
                const defaultProbability = (1.0 - sumProbability) / numNoProbability;

                // todo: make sameO
                const ops = [];
                let offset = 0.0;
                for (const i in operations) {
                    const op = operations[i];
                    if (i === operations.length - 1) {
                        ops.push(new Operation(1.0, op.body));
                    } else {
                        const probability = op.probability || defaultProbability;
                        ops.push(new Operation(offset + probability, op.body));
                        offset += probability;
                    }
                }

                if (!productions[symbolName]) {
                    productions[symbolName] = [];
                }
                productions[symbolName].push(new Production({
                    symbol: spec.symbol,
                    predecessors: spec.predecessors,
                    successors: spec.successors,
                    condition: spec.condition.func,
                    operations: ops,
                }));

                processed.add(i);
            }

            return productions;
        }, {});
    }

    #parseProductionCondition(conditionDefinition, moduleParameters, systemParameters) {
        const requiredSystemParameters = [];
        for (const p of Object.keys(systemParameters)) {
            if (conditionDefinition.includes(p)) {
                requiredSystemParameters.push(p);
            }
        }
        const systemParamsString = `{${
            requiredSystemParameters.map(p => `${p}=${systemParameters[p]}`)
        }`;
        const funcArgs = [...moduleParameters];
        if (Object.keys(requiredSystemParameters).length) {
            funcArgs.push(systemParamsString);
        }
        return {
            func: conditionDefinition.length ? new Function(
                ...funcArgs,
                `return !!(${conditionDefinition})`,
            ) : null,
            conditionDefinition,
        };
    }

    #parseProductionBody(bodyDefinition, moduleParameters, systemParameters, symbols) {
        const producedSymbols = LSystemParser.#parseProductionPreOrSuccessors(bodyDefinition, symbols);
        const body = producedSymbols.map(s => {
            return `new ctor({ name: '${s.name}', parameters: [${s.parameters}] })`
        });
        const funcArgs = ['ctor', ...moduleParameters];
        if (Object.keys(systemParameters).length) {
            funcArgs.push(`{${Object.keys(systemParameters).map(k =>`${k}=${systemParameters[k]}`)}}`);
        }
        return new Function(
            ...funcArgs,
            `return [${body}];`
        );
    }

    // todo: same as parseAxiom except for Symbol instead of Module -> combine and add if statement
    static #parseProductionPreOrSuccessors(moduleDefinitions, symbols) {
        const parsedSymbols = [];
        const originalAxiom = `${moduleDefinitions}`;
        while (moduleDefinitions.length) {
            let found = false;
            for (const s of symbols) {
                if (moduleDefinitions.indexOf(s.name) === 0) {
                    const openIdx = moduleDefinitions.indexOf('(');
                    const closeIdx = findClosingParenthesis(moduleDefinitions, openIdx);
                    if (openIdx === s.name.length) {
                        const moduleDefinition = moduleDefinitions.slice(0, closeIdx + 1);
                        if (s.equals(Symbol.fromString(moduleDefinition))) {
                            found = true;
                            parsedSymbols.push(Symbol.fromString(moduleDefinition));
                            moduleDefinitions = moduleDefinitions.slice(moduleDefinition.length);
                        }
                    } else if (s.parameters.length === 0) {
                        found = true;
                        parsedSymbols.push(new Symbol({
                            name: s.name,
                            parameters: [],
                        }));
                        moduleDefinitions = moduleDefinitions.slice(s.name.length);
                    }
                }
                if (found) {
                    break;
                }
            }
            if (!found) {
                throw Error(`Incomplete alphabet ${symbols.map(s => s.toString())} for modules ${originalAxiom}`);
            }
        }
        return parsedSymbols;
    }

    static #parseAxiom(axiom, symbols) {
        const parsedAxiom = [];
        const originalAxiom = `${axiom}`;
        while (axiom.length) {
            let found = false;
            for (const s of symbols) {
                if (axiom.indexOf(s.name) === 0) {
                    const openIdx = axiom.indexOf('(');
                    const closeIdx = findClosingParenthesis(axiom, openIdx);
                    if (openIdx === s.name.length) {
                        const moduleDefinition = axiom.slice(0, closeIdx + 1);
                        if (s.equals(Symbol.fromString(moduleDefinition))) {
                            found = true;
                            parsedAxiom.push(Module.fromString(moduleDefinition));
                            axiom = axiom.slice(moduleDefinition.length);
                        }
                    } else if (s.parameters.length === 0) {
                        found = true;
                        parsedAxiom.push(new Module({
                            name: s.name,
                            parameters: [],
                        }));
                        axiom = axiom.slice(s.name.length);
                    }
                }
                if (found) {
                    break;
                }
            }
            if (!found) {
                throw Error(`Incomplete alphabet ${symbols.map(s => s.toString())} for axiom ${originalAxiom}`);
            }
        }
        return parsedAxiom;
    }

    #splitModuleSpecification(definition) {
        const predSeparator = definition.indexOf(this.#modulePredecessorSeparator);
        const succSeparator = definition.indexOf(this.#moduleSuccessorSeparator);
        return {
            predecessors: predSeparator < 0 ? '' : definition.slice(0, predSeparator),
            successors: succSeparator < 0 ? '' : definition.slice(succSeparator + 1),
            module: Symbol.fromString(
                definition.slice(predSeparator + 1, succSeparator < 0 ? definition.length : succSeparator)
            )
        }
    }

    #splitProductionDefinition(definition) {
        const cleanDefinition = definition.replace(/\s/g, '');
        let probSepIdx = cleanDefinition.indexOf(this.#probabilitySeparator);
        let condSepIdx = cleanDefinition.indexOf(this.#conditionSeparator);
        let opSepIdx = cleanDefinition.indexOf(this.#operationSeparator);

        console.assert(opSepIdx > 0);
        console.assert(probSepIdx < condSepIdx < opSepIdx);

        // probabilities & conditions are optional
        if (condSepIdx < 0) condSepIdx = opSepIdx;

        return {
            probability: probSepIdx < 0 ? null : Number(cleanDefinition.slice(0, probSepIdx)),
            moduleSpecification: this.#splitModuleSpecification(cleanDefinition.slice(probSepIdx + 1, condSepIdx)),
            condition: cleanDefinition.slice(condSepIdx + 1, opSepIdx),
            body: cleanDefinition.slice(opSepIdx + 2)
        };
    }
}

export class LSystem {
    #symbols;
    #parameters;
    #axiom;
    #productions;

    constructor({symbols, parameters, axiom, productions}) {
        this.#symbols = symbols;
        this.#parameters = parameters;
        this.#axiom = axiom;
        this.#productions = productions;
    }

    evaluate(systemState) {
        const result = [];
        for (const i in systemState.axiom) {
            const m = systemState.axiom[i];
            const p = this.#findProduction(i, systemState);
            result.push(p ? p.apply(m, systemState) : m.clone());
        }
        systemState.axiom = result.flat();
        return systemState;
    }

    #getProductionsForModule(m) {
        return this.#productions[m.toGenericString()];
    }

    #findProduction(i, systemState) {
        const modules = systemState.axiom;
        const module = modules[i];
        const candidates = this.#getProductionsForModule(module);
        if (!candidates) {
            return null;
        }
        candidates.sort((a, b) => b.rank() - a.rank());
        for (const c of candidates) {
            let matches = true;
            for (const j in c.predecessors) {
                if (i - j - 1 < 0) {
                    matches = false;
                    break;
                }
                if (!c.predecessors[c.predecessors.length - j - 1].equals(modules[i - j - 1])) {
                    matches = false;
                    break;
                }
            }
            if (matches) {
                for (const j in c.successors) {
                    if (i + j < modules.length) {
                        matches = false;
                        break;
                    }
                    if (!c.successors[j].equals(modules[i + j])) {
                        matches = false;
                        break;
                    }
                }
            }
            if (matches && c.condition(module, systemState)) {
                return c;
            }
        }
        return null;
    }

    get axiom() {
        return this.#axiom.map(m => m.clone());
    }

    get parameters() {
        return JSON.parse(JSON.stringify(this.#parameters));
    }
}

export class LSystemIterator {
    #lSystem;
    #state;
    #previousStates;

    constructor(lSystem) {
        this.#lSystem = lSystem;
        this.#state = new LSystemState({ axiom: lSystem.axiom, parameters: lSystem.parameters });
        this.#previousStates = [];
    }

    next(asString = true) {
        this.#previousStates = this.#state.clone();
        this.state = this.#lSystem.evaluate(this.#state);
        return asString ? this.state.toString() : this.state.axiom;
    }

    // todo: this is an infinite loop?
    nth(i, asString = true) {
        if (this.#previousStates.length > i) {
            const state = this.#previousStates[i];
            return asString ? state.toString() : state.axiom;
        }
        while (i !== this.#previousStates.length) {
            this.next();
        }
        return asString ? this.state.toString() : this.state.axiom;
    }

    static createLSystemIterator({alphabet = undefined, parameters = {}, productions, axiom}) {
        const lSystem = new LSystemParser({})
            .parseLSystem({
                alphabet,
                parameters,
                productions,
                axiom,
        });
        return new LSystemIterator(lSystem);
    }
}

function runLSystem(definition) {
    const lSystem = new LSystemParser({}).parseLSystem(definition);
    const evaluator = new LSystemIterator(lSystem);
    console.log(definition);
    console.log(lSystem);
    for (const r of definition.results) {
        const result = evaluator.next();
        console.log(r);
        console.log(result);
    }
    if (!definition.results.length) {
        for (let i = 0; i < 5; ++i) {
            console.log(evaluator.next());
        }
    }
    return lSystem;
}

export function testLSystems() {
    const deterministic = {
        axiom: 'a',
        productions: [
            'a->ab',
            'b->ac'
        ],
        parameters: {},
        alphabet: ['a', 'b', 'c'],
        results: [
            'ab',
            'abac',
            'abacabc',
            'abacabcabacc'
        ]
    }

    const stochastic = {
        axiom: 'F',
        productions: [
            'F->F[+F]F[-F]F',
            'F->F[+F]F',
            'F->F[-F]F'
        ],
        parameters: {},
        alphabet: ['F', '[', ']', '+', '-'],
        results: []
    }

    const contextSensitive = {
        axiom: 'baaaaaaa',
        productions: [
            'b<a -> b',
            'b->a'
        ],
        parameters: {},
        alphabet: ['a', 'b'],
        results: [
            'abaaaaaa',
            'aabaaaaa',
            'aaabaaaa',
            'aaaabaaa',
            'aaaaabaa',
            'aaaaaaba'
        ]
    }

    const parametric = {
        axiom: 'B(2)A(4,4)',
        productions: [
            'A(x,y): y<=3 -> A(x*2,x+y)',
            'A(x,y): y> 3 -> B(x)A(x/y,0)',
            'B(x)  : x< 1 -> C',
            'B(x)  : x>=1 -> B(x-1)'
        ],
        parameters: {},
        alphabet: ['A(x,y)', 'B(x)', 'C'],
        results: [
            'B(1)B(4)A(1,0)',
            'B(0)B(3)A(2,1)',
            'CB(2)A(4,3)',
            'CB(1)A(8,7)',
            'CB(0)B(8)A(1.142,0)'
        ]
    }

    const treeModel = {
        axiom: 'A(1,10)',
        productions: [
            'A(l,w) -> F(l,w)[&(a0)B(l*r2,w*wr)]/(d)A(l*r1,w*wr)',
            'B(l,w) -> F(l,w)[+(-a1)$C(l*r2,w*wr)]C(l*r1,w*wr)',
            'C(l,w) -> F(l,w)[+(a1)$B(l*r2,w*wr)]B(l*r1,w*wr)'
        ],
        parameters: {r1: 7.0, r2: 7.1, a0: 7.2, a1: 7.3, d: 7.4, wr: 7.5},
        alphabet: ['A(l,w)', 'B(l,w)', 'C(l,w)', 'F(l,w)', '[', ']', '+(a)', '&(a)', '/(d)', '$'],
        results: []
    };

    const queryParameters = {
        axiom: 'A',
        productions: [
            'A->F(1)?P(x,y)-A',
            'F(k)->F(k+1)'
        ],
        parameters: {},
        alphabet: ['A', 'F(k)', '?P(x,y)', '-'],
        results: [
            'F(1)?P(0,1)-A',
            'F(2)?P(0,2)-F(1)?P(1,2)-A',
            'F(3)?P(0,3)-F(1)?P(2,3)-F(1)?P(2,2)-A'
        ],
        interpreter: {
            'F': (k, ctx) => {
                const result = {...ctx};
                const distance = k * ctx.dir;
                if (ctx.axis === 'x') {
                    result.x += distance;
                } else {
                    result.y += distance;
                }
                return result;
            }
        }
    };

    const definitions = [
        deterministic,
        stochastic,
        contextSensitive,
        parametric,
        treeModel,
        queryParameters
    ];
    const systems = [];
    for (const d of definitions) {
        systems.push(runLSystem(d));
    }
    return systems;
}
