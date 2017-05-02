_Bool 
    transaction()
{
 
_Bool 
     volatile result;

 { jmp_buf _tx_env; int _stage; int _pobj_errno; if (
setjmp
(_tx_env)) { 
(*__errno_location()) 
= pmemobj_tx_errno(); } else { _pobj_errno = pmemobj_tx_begin(pop, _tx_env, TX_PARAM_NONE, TX_PARAM_NONE); if (_pobj_errno) 
(*__errno_location()) 
= _pobj_errno; } while ((_stage = pmemobj_tx_stage()) != TX_STAGE_NONE) { switch (_stage) { case TX_STAGE_WORK: {



  fputs("Transaction Begin - do work here", 
                                           (stderr)
                                                 );

 } pmemobj_tx_process(); break; case TX_STAGE_ONCOMMIT: {



  fputs("Transaction Commit", 
                             (stderr)
                                   );
  result = 
          1
              ;

 } pmemobj_tx_process(); break; case TX_STAGE_ONABORT: {



  fputs("Transaction Abort", 
                            (stderr)
                                  );
  result = 
          0
               ;

 } pmemobj_tx_process(); break; case TX_STAGE_FINALLY: {




  fputs("Transaction Finally", 
                              (stderr)
                                    );

 } pmemobj_tx_process(); break; default: do {} while (0); pmemobj_tx_process(); break; } } _pobj_errno = pmemobj_tx_end(); if (_pobj_errno) 
  (*__errno_location()) 
  = _pobj_errno;}



 return result;
}


_Bool 
    transaction2()
{
 int * volatile good_example = (int *)0xBAADF00D;

 
_Bool 
     volatile result;

 { jmp_buf _tx_env; int _stage; int _pobj_errno; if (
setjmp
(_tx_env)) { 
(*__errno_location()) 
= pmemobj_tx_errno(); } else { _pobj_errno = pmemobj_tx_begin(pop, _tx_env, TX_PARAM_NONE, TX_PARAM_NONE); if (_pobj_errno) 
(*__errno_location()) 
= _pobj_errno; } while ((_stage = pmemobj_tx_stage()) != TX_STAGE_NONE) { switch (_stage) { case TX_STAGE_WORK: {



  fputs("Transaction Begin - do work here", 
                                           (stderr)
                                                 );

  good_example = malloc(sizeof(int));

 } pmemobj_tx_process(); break; case TX_STAGE_ONCOMMIT: {



  fputs("Transaction Commit", 
                             (stderr)
                                   );
  result = 
          1
              ;

  free(good_example);

 } pmemobj_tx_process(); break; case TX_STAGE_ONABORT: {



  fputs("Transaction Abort", 
                            (stderr)
                                  );
  result = 
          0
               ;

 } pmemobj_tx_process(); break; case TX_STAGE_FINALLY: {




  fputs("Transaction Finally", 
                              (stderr)
                                    );

 } pmemobj_tx_process(); break; default: do {} while (0); pmemobj_tx_process(); break; } } _pobj_errno = pmemobj_tx_end(); if (_pobj_errno) 
  (*__errno_location()) 
  = _pobj_errno;}


 return result;
}
