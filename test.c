
// Build with `gcc -lddcutil test.c`

#include<ddcutil_types.h>
#include<ddcutil_c_api.h>
#include<ddcutil_status_codes.h>
#include<ddcutil_macros.h>


int main() {

    DDCA_Display_Identifier did;
    int rc = ddca_create_busno_display_identifier(6, &did);

    if (rc != 0) {
        printf("bad display identifier! %d\n", rc);
    }

    printf("did = %s\n", ddca_did_repr(did));

    DDCA_Display_Ref dref;
    rc = ddca_get_display_ref(did, &dref);
    // rc = ddca_create_display_ref(did, &dref);
    if (rc != 0) {
        printf("bad display reference! %d\n", rc);
    }

    printf("dref = %s\n", ddca_dref_repr(dref));
    // invalid? :/
}