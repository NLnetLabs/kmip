//! See: https://docs.oasis-open.org/kmip/testcases/v1.1/cn01/kmip-testcases-v1.1-cn01.html#_Toc333488803

#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};

use krill_kmip_ttlv::ser::to_vec;

use crate::{
    response::from_slice,
    types::{
        common::{ObjectType, Operation, UniqueBatchItemID},
        request::{
            self, Authentication, BatchCount, BatchItem, MaximumResponseSize, ProtocolVersionMajor,
            ProtocolVersionMinor, QueryFunction, RequestHeader, RequestMessage, RequestPayload,
        },
        response::{ResponseMessage, ResponsePayload, ResultReason, ResultStatus},
    },
};

#[test]
fn query_request_operations_objects_max_response_size_256() {
    #[rustfmt::skip]
    let use_case_request = 
        RequestMessage(                             // Tag: 0x420078, Type: 0x01 (Structure)
            RequestHeader(                          //   Tag: 0x420077, Type: 0x01 (Structure)
                request::ProtocolVersion(           //     Tag: 0x420069, Type: 0x01 (Structure)
                    ProtocolVersionMajor(1),        //       Tag: 0x42006A, Type: 0x02 (Integer), Data: 0x00000001 (1)
                    ProtocolVersionMinor(1)         //       Tag: 0x42006B, Type: 0x02 (Integer), Data: 0x00000000 (0)
                ),                                  //
                Some(MaximumResponseSize(256)),     //     Tag: 0x420050, Type: 0x02 (Integer), Data: 0x00000100 (256)
                Option::<Authentication>::None,     //
                BatchCount(1),                      //     Tag: 0x42000D, Type: 0x02 (Integer), Data: 0x00000001 (1)
            ),                                      //
            vec![BatchItem(                         //   Tag: 0x42000F, Type: 0x01 (Structure)
                Operation::Query,                   //     Tag: 0x42005C, Type: 0x05 (Enumeration). Data: 0x00000018
                Option::<UniqueBatchItemID>::None,  //
                RequestPayload::Query(vec![         //     Tag: 0x420079, Type: 0x01 (Structure)
                    QueryFunction::QueryOperations, //       Tag: 0x420074, Type: 0x05 (Enumeration), Data: 0x00000001
                    QueryFunction::QueryObjects     //       Tag: 0x420074, Type: 0x05 (Enumeration), Data: 0x00000002
                ]),
            )],
        );

    #[rustfmt::skip]
    let use_case_request_hex = concat!(
        "42007801000000904200770100000048420069010000002042006A0200000004000000010000000042006B02000000040",
    //   ^RequestMessage ^RequestHeader  ^ProtocolVersion^ProtocolVersionMajor           ^ProtocolVersionMinor
        "0000001000000004200500200000004000001000000000042000D0200000004000000010000000042000F010000003842",
    //                  ^MaximumResponseSize            ^BatchCount                     ^BatchItem      ^O
        "005C050000000400000018000000004200790100000020420074050000000400000001000000004200740500000004000",
    //   peration                      ^RequestPayload ^QueryFunction::Operations      ^QueryFunction::Objects
        "0000200000000"
    );
    let actual_request_hex = hex::encode_upper(to_vec(&use_case_request).unwrap());

    assert_eq!(use_case_request_hex, actual_request_hex);
}

#[test]
fn query_response_operation_failed_response_too_large() {
    let use_case_response_hex = concat!(
        "42007B01000000C842007A0100000048420069010000002042006A0200000004000000010000000042006B02000000040",
        "0000001000000004200920900000008000000004F9A556B42000D0200000004000000010000000042000F010000007042",
        "007F0500000004000000010000000042007E0500000004000000020000000042007D0700000043526573706F6E7365207",
        "3697A653A203634382C204D6178696D756D20526573706F6E73652053697A6520696E6469636174656420696E20726571",
        "756573743A203235360000000000",
    );
    let ttlv_wire = hex::decode(use_case_response_hex).unwrap();
    let res: ResponseMessage = from_slice(ttlv_wire.as_ref()).unwrap();

    assert_eq!(res.header.protocol_version.major, 1);
    assert_eq!(res.header.protocol_version.minor, 1);
    assert_eq!(res.header.timestamp, 0x000000004F9A556B);
    assert_eq!(res.header.batch_count, 1);
    assert_eq!(res.batch_items.len(), 1);

    let item = &res.batch_items[0];
    assert!(matches!(item.result_status, ResultStatus::OperationFailed));
    assert!(matches!(item.result_reason, Some(ResultReason::ResponseTooLarge)));
    assert_eq!(
        item.result_message,
        Some("Response size: 648, Maximum Response Size indicated in request: 256".to_string())
    );
}

#[test]
fn query_request_operations_objects_max_response_size_2048() {
    #[rustfmt::skip]
    let use_case_request = 
        RequestMessage(                             // Tag: 0x420078, Type: 0x01 (Structure)
            RequestHeader(                          //   Tag: 0x420077, Type: 0x01 (Structure)
                request::ProtocolVersion(           //     Tag: 0x420069, Type: 0x01 (Structure)
                    ProtocolVersionMajor(1),        //       Tag: 0x42006A, Type: 0x02 (Integer), Data: 0x00000001 (1)
                    ProtocolVersionMinor(1)         //       Tag: 0x42006B, Type: 0x02 (Integer), Data: 0x00000000 (0)
                ),                                  //
                Some(MaximumResponseSize(2048)),    //     Tag: 0x420050, Type: 0x02 (Integer), Data: 0x00000800 (2048)
                Option::<Authentication>::None,     //
                BatchCount(1),                      //     Tag: 0x42000D, Type: 0x02 (Integer), Data: 0x00000001 (1)
            ),                                      //
            vec![BatchItem(                         //   Tag: 0x42000F, Type: 0x01 (Structure)
                Operation::Query,                   //     Tag: 0x42005C, Type: 0x05 (Enumeration). Data: 0x00000018
                Option::<UniqueBatchItemID>::None,  //
                RequestPayload::Query(vec![         //     Tag: 0x420079, Type: 0x01 (Structure)
                    QueryFunction::QueryOperations, //       Tag: 0x420074, Type: 0x05 (Enumeration), Data: 0x00000001
                    QueryFunction::QueryObjects,    //       Tag: 0x420074, Type: 0x05 (Enumeration), Data: 0x00000002
                    QueryFunction::QueryServerInformation,// Tag: 0x420074, Type: 0x05 (Enumeration), Data: 0x00000003
                ]),
            )],
        );

    #[rustfmt::skip]
    let use_case_request_hex = concat!(
        "42007801000000A04200770100000048420069010000002042006A0200000004000000010000000042006B02000000040",
    //   ^RequestMessage ^RequestHeader  ^ProtocolVersion^ProtocolVersionMajor           ^ProtocolVersionMinor
        "0000001000000004200500200000004000008000000000042000D0200000004000000010000000042000F010000004842",
    //                  ^MaximumResponseSize            ^BatchCount                     ^BatchItem      ^O
        "005C050000000400000018000000004200790100000030420074050000000400000001000000004200740500000004000",
    //   peration                      ^RequestPayload ^QueryFunction::Operations      ^QueryFunction::Objects
        "000020000000042007405000000040000000300000000"
    //                ^QueryFunction::ServerInformation
    );
    let actual_request_hex = hex::encode_upper(to_vec(&use_case_request).unwrap());

    assert_eq!(use_case_request_hex, actual_request_hex);
}

#[test]
fn query_response_operation_succeeded() {
    let use_case_response_hex = concat!(
        "42007B01000002C042007A0100000048420069010000002042006A0200000004000000010000000042006B02000000040",
        "0000001000000004200920900000008000000004F9A556B42000D0200000004000000010000000042000F010000026842",
        "005C0500000004000000180000000042007F0500000004000000000000000042007C010000024042005C0500000004000",
        "000010000000042005C0500000004000000020000000042005C0500000004000000030000000042005C05000000040000",
        "00040000000042005C0500000004000000060000000042005C0500000004000000070000000042005C050000000400000",
        "0080000000042005C0500000004000000090000000042005C05000000040000000A0000000042005C0500000004000000",
        "0B0000000042005C05000000040000000C0000000042005C05000000040000000D0000000042005C05000000040000000",
        "E0000000042005C05000000040000000F0000000042005C0500000004000000100000000042005C050000000400000011",
        "0000000042005C0500000004000000120000000042005C0500000004000000130000000042005C0500000004000000140",
        "000000042005C0500000004000000150000000042005C0500000004000000160000000042005C05000000040000001800",
        "00000042005C0500000004000000190000000042005C05000000040000001A0000000042005C05000000040000001D000",
        "0000042005C05000000040000001E00000000420057050000000400000001000000004200570500000004000000020000",
        "0000420057050000000400000003000000004200570500000004000000040000000042005705000000040000000600000",
        "0004200570500000004000000070000000042009D070000002E49424D2074657374207365727665722C206E6F742D544B",
        "4C4D20322E302E312E31204B4D495020322E302E302E3100004200880100000000"
    );
    let ttlv_wire = hex::decode(use_case_response_hex).unwrap();
    let res: ResponseMessage = from_slice(ttlv_wire.as_ref()).unwrap();

    assert_eq!(res.header.protocol_version.major, 1);
    assert_eq!(res.header.protocol_version.minor, 1);
    assert_eq!(res.header.timestamp, 0x000000004F9A556B);
    assert_eq!(res.header.batch_count, 1);
    assert_eq!(res.batch_items.len(), 1);

    let item = &res.batch_items[0];
    assert!(matches!(item.result_status, ResultStatus::Success));
    assert!(matches!(item.operation, Some(Operation::Query)));
    assert!(matches!(&item.payload, Some(ResponsePayload::Query(_))));

    if let Some(ResponsePayload::Query(payload)) = item.payload.as_ref() {
        assert_eq!(payload.operations.len(), 26);
        assert!(matches!(payload.operations[0], Operation::Create));
        assert!(matches!(payload.operations[1], Operation::CreateKeyPair));
        assert!(matches!(payload.operations[2], Operation::Register));
        assert!(matches!(payload.operations[3], Operation::Rekey));
        assert!(matches!(payload.operations[4], Operation::Certify));
        assert!(matches!(payload.operations[5], Operation::Recertify));
        assert!(matches!(payload.operations[6], Operation::Locate));
        assert!(matches!(payload.operations[7], Operation::Check));
        assert!(matches!(payload.operations[8], Operation::Get));
        assert!(matches!(payload.operations[9], Operation::GetAttributes));
        assert!(matches!(payload.operations[10], Operation::GetAttributeList));
        assert!(matches!(payload.operations[11], Operation::AddAttribute));
        assert!(matches!(payload.operations[12], Operation::ModifyAttribute));
        assert!(matches!(payload.operations[13], Operation::DeleteAttribute));
        assert!(matches!(payload.operations[14], Operation::ObtainLease));
        assert!(matches!(payload.operations[15], Operation::GetUsageAllocation));
        assert!(matches!(payload.operations[16], Operation::Activate));
        assert!(matches!(payload.operations[17], Operation::Revoke));
        assert!(matches!(payload.operations[18], Operation::Destroy));
        assert!(matches!(payload.operations[19], Operation::Archive));
        assert!(matches!(payload.operations[20], Operation::Recover));
        assert!(matches!(payload.operations[21], Operation::Query));
        assert!(matches!(payload.operations[22], Operation::Cancel));
        assert!(matches!(payload.operations[23], Operation::Poll));
        assert!(matches!(payload.operations[24], Operation::RekeyKeyPair));
        assert!(matches!(payload.operations[25], Operation::DiscoverVersions));

        assert_eq!(payload.object_types.len(), 6);
        assert!(matches!(payload.object_types[0], ObjectType::Certificate));
        assert!(matches!(payload.object_types[1], ObjectType::SymmetricKey));
        assert!(matches!(payload.object_types[2], ObjectType::PublicKey));
        assert!(matches!(payload.object_types[3], ObjectType::PrivateKey));
        assert!(matches!(payload.object_types[4], ObjectType::Template));
        assert!(matches!(payload.object_types[5], ObjectType::SecretData));

        assert!(payload.operations.contains(&Operation::Cancel));
        assert!(payload.object_types.contains(&ObjectType::Template));

        assert_eq!(
            payload.vendor_identification,
            Some("IBM test server, not-TKLM 2.0.1.1 KMIP 2.0.0.1".into())
        );
        assert!(payload.server_information.is_some());
    } else {
        panic!("Wrong payload");
    }
}