# TDF_LabelNode usage and binding investigation in OCCT

- **Date:** 2026-02-18 15:15 – 17:16
- **Model:** Claude Opus 4.6
- **Turns:** 2
- **Tool calls:** 274
- **Thinking blocks:** 5
- **Input words:** 55,227
- **Output words:** 7,486
- **Prompt tokens (last round):** 109,479
- **API rounds:** 110
- **Total elapsed:** 4307s

## Table of Contents

1. [Investigate how TDF_LabelNode is used in OCCT. My guess is that some other API generates them, an...](#user-1) (Claude Opus 4.6)
2. [@agent Try Again](#user-2) (Claude Opus 4.6)

---

<a id="user-1"></a>

## User (1)

> Investigate how TDF_LabelNode is used in OCCT. My guess is that some other API generates them, and client code is expected to explore them via pointers, but not directly create or destroy them. Would it be useful to have bindings to them? If so, I think we need to be able to generate bindings for classes that can&#x27;t be constructed or destructed, and just omit the features that require constructability/destructability. If you agree, implement a fix and commit.

*2026-02-18 15:16*

### Assistant — Claude Opus 4.6

> 💭 Let me investigate TDF_LabelNode in OCCT to understand how it's used, whether it's useful to have bindings, and what would be needed to support classes that can't be constructed/destructed.

**Todo list:**
- 🔄 Investigate TDF_LabelNode usage in OCCT
- ⬜ Check current generator handling
- ⬜ Implement non-constructible class support
- ⬜ Test and validate changes
- ⬜ Commit changes

Research TDF_LabelNode in OCCT

Read [TDF_LabelNode.hxx](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx)

<details>
<summary>Searched for text <code>TDF_LabelNode</code> (<code>**/crates/occt-sys/OCCT/src/TDF/*.hxx</code>), 61 results</summary>

- [TDF_LabelNode.hxx:16](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L16)
- [TDF_LabelNode.hxx:17](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L17)
- [TDF_LabelNode.hxx:21](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L21)
- [TDF_LabelNode.hxx:36](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L36)
- [TDF_LabelNode.hxx:37](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L37)
- [TDF_LabelNode.hxx:38](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L38)
- [TDF_LabelNode.hxx:39](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L39)
- [TDF_LabelNode.hxx:43](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L43)
- [TDF_LabelNode.hxx:46](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L46)
- [TDF_LabelNode.hxx:54](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L54)
- [TDF_LabelNode.hxx:57](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L57)
- [TDF_LabelNode.hxx:60](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L60)
- [TDF_LabelNode.hxx:69](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L69)
- [TDF_LabelNode.hxx:80](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L80)
- [TDF_LabelNode.hxx:87](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L87)
- [TDF_LabelNode.hxx:93](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L93)
- [TDF_LabelNode.hxx:98](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L98)
- [TDF_LabelNode.hxx:106](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L106)
- [TDF_LabelNode.hxx:122](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L122)
- [TDF_LabelNode.hxx:129](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L129)
- [TDF_LabelNode.hxx:131](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L131)
- [TDF_LabelNode.hxx:141](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L141)
- [TDF_LabelNode.hxx:147](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L147)
- [TDF_LabelNode.hxx:150](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L150)
- [TDF_LabelNode.hxx:155](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L155)
- [TDF_LabelNode.hxx:156](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L156)
- [TDF_LabelNode.hxx:157](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L157)
- [TDF_LabelNode.hxx:158](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L158)
- [TDF_Label.hxx:24](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L24)
- [TDF_Label.hxx:267](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L267)
- [TDF_Label.hxx:271](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L271)
- [TDF_Label.hxx:277](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L277)
- [TDF_Label.hxx:282](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L282)
- [TDF_Label.hxx:285](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L285)
- [TDF_Label.hxx:293](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L293)
- [TDF_AttributeIterator.hxx:26](../crates/occt-sys/OCCT/src/TDF/TDF_AttributeIterator.hxx#L26)
- [TDF_AttributeIterator.hxx:52](../crates/occt-sys/OCCT/src/TDF/TDF_AttributeIterator.hxx#L52)
- [TDF_Data.hxx:22](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L22)
- [TDF_Data.hxx:117](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L117)
- [TDF_Data.hxx:119](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L119)
- [TDF_Data.hxx:136](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L136)
- [TDF_Data.hxx:139](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L139)
- [TDF_Data.hxx:153](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L153)
- [TDF_Data.hxx:204](../crates/occt-sys/OCCT/src/TDF/TDF_Data.hxx#L204)
- [TDF_ChildIterator.hxx:98](../crates/occt-sys/OCCT/src/TDF/TDF_ChildIterator.hxx#L98)
- [TDF_Attribute.hxx:22](../crates/occt-sys/OCCT/src/TDF/TDF_Attribute.hxx#L22)
- [TDF_Attribute.hxx:382](../crates/occt-sys/OCCT/src/TDF/TDF_Attribute.hxx#L382)
- [TDF_Attribute.hxx:405](../crates/occt-sys/OCCT/src/TDF/TDF_Attribute.hxx#L405)
- [TDF_LabelNodePtr.hxx:16](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx#L16)
- [TDF_LabelNodePtr.hxx:17](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx#L17)
- [TDF_LabelNodePtr.hxx:19](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx#L19)
- [TDF_LabelNodePtr.hxx:20](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx#L20)
- [TDF_LabelNodePtr.hxx:22](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx#L22)
</details>

<details>
<summary>Searched for text <code>TDF_LabelNode</code> (<code>**/crates/occt-sys/OCCT/src/TDF/*.cxx</code>), 43 results</summary>

- [TDF_ChildIterator.cxx:23](../crates/occt-sys/OCCT/src/TDF/TDF_ChildIterator.cxx#L23)
- [TDF_Attribute.cxx:40](../crates/occt-sys/OCCT/src/TDF/TDF_Attribute.cxx#L40)
- [TDF_ChildIDIterator.cxx:24](../crates/occt-sys/OCCT/src/TDF/TDF_ChildIDIterator.cxx#L24)
- [TDF_AttributeIterator.cxx:46](../crates/occt-sys/OCCT/src/TDF/TDF_AttributeIterator.cxx#L46)
- [TDF_LabelNode.cxx:16](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L16)
- [TDF_LabelNode.cxx:22](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L22)
- [TDF_LabelNode.cxx:26](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L26)
- [TDF_LabelNode.cxx:31](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L31)
- [TDF_LabelNode.cxx:48](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L48)
- [TDF_LabelNode.cxx:75](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L75)
- [TDF_LabelNode.cxx:81](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L81)
- [TDF_LabelNode.cxx:85](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L85)
- [TDF_LabelNode.cxx:98](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L98)
- [TDF_LabelNode.cxx:120](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L120)
- [TDF_LabelNode.cxx:142](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L142)
- [TDF_LabelNode.cxx:147](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L147)
- [TDF_LabelNode.cxx:159](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L159)
- [TDF_LabelNode.cxx:164](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L164)
- [TDF_LabelNode.cxx:173](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L173)
- [TDF_LabelNode.cxx:178](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L178)
- [TDF_LabelNode.cxx:185](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx#L185)
- [TDF_Label.cxx:24](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L24)
- [TDF_Label.cxx:25](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L25)
- [TDF_Label.cxx:115](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L115)
- [TDF_Label.cxx:116](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L116)
- [TDF_Label.cxx:167](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L167)
- [TDF_Label.cxx:285](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L285)
- [TDF_Label.cxx:288](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L288)
- [TDF_Label.cxx:289](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L289)
- [TDF_Label.cxx:290](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L290)
- [TDF_Label.cxx:291](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L291)
- [TDF_Label.cxx:327](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L327)
- [TDF_Label.cxx:521](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L521)
- [TDF_Label.cxx:565](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L565)
- [TDF_Label.cxx:628](../crates/occt-sys/OCCT/src/TDF/TDF_Label.cxx#L628)
- [TDF_Data.cxx:33](../crates/occt-sys/OCCT/src/TDF/TDF_Data.cxx#L33)
- [TDF_Data.cxx:52](../crates/occt-sys/OCCT/src/TDF/TDF_Data.cxx#L52)
- [TDF_Data.cxx:106](../crates/occt-sys/OCCT/src/TDF/TDF_Data.cxx#L106)
</details>

Searched for files matching `**/crates/occt-sys/OCCT/src/TDF/TDF_LabelNode*`, 3 matches
- [TDF_LabelNodePtr.hxx](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx)
- [TDF_LabelNode.hxx](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx)
- [TDF_LabelNode.cxx](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx)

Read [TDF_LabelNode.cxx](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.cxx)

Read [TDF_Label.hxx](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#250-250), lines 250 to 299

Read [TDF_LabelNodePtr.hxx](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNodePtr.hxx)

<details>
<summary>Searched for text <code>TDF_LabelNode</code> (<code>**/crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx</code>), 7 results</summary>

- [TDF_Label.hxx:24](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L24)
- [TDF_Label.hxx:267](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L267)
- [TDF_Label.hxx:271](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L271)
- [TDF_Label.hxx:277](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L277)
- [TDF_Label.hxx:282](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L282)
- [TDF_Label.hxx:285](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L285)
- [TDF_Label.hxx:293](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L293)
</details>

<details>
<summary>Searched for regex <code>public:|LabelNode|private:|protected:</code> (<code>**/crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx</code>), 12 results</summary>

- [TDF_Label.hxx:24](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L24)
- [TDF_Label.hxx:56](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L56)
- [TDF_Label.hxx:90](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L90)
- [TDF_Label.hxx:264](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L264)
- [TDF_Label.hxx:265](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L265)
- [TDF_Label.hxx:267](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L267)
- [TDF_Label.hxx:271](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L271)
- [TDF_Label.hxx:277](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L277)
- [TDF_Label.hxx:282](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L282)
- [TDF_Label.hxx:285](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L285)
- [TDF_Label.hxx:293](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#L293)
</details>

Read [TDF_Label.hxx](../crates/occt-sys/OCCT/src/TDF/TDF_Label.hxx#1-1), lines 1 to 260

Searched for regex `LabelNode.*public|public.*LabelNode` (`**/crates/occt-sys/OCCT/src/TDF/TDF_Attribute.hxx`), no results

Searched for text `DEFINE_NCOLLECTION_ALLOC` (`**/crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx`), 2 results
- [NCollection_DefineAlloc.hxx:25](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L25)
- [NCollection_DefineAlloc.hxx:31](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L31)

Read [NCollection_DefineAlloc.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#15-15), lines 15 to 43

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- ⬜ Check for other excluded non-constructible classes
- ⬜ Implement non-constructible class support
- ⬜ Test and validate changes
- ⬜ Commit changes

Based on the research, **TDF_LabelNode would not be useful to bind**. Here's why:

1. **No public API returns `TDF_LabelNode*`** — It's always hidden behind private members in `TDF_Label`, `TDF_Data`, and `TDF_Attribute`. Client code never touches it.

2. **`TDF_Label` already exposes all the same information** — `Tag()`, `Depth()`, `Father()`, `IsRoot()`, `Data()`, `HasChild()`, `NbChildren()`, `FindChild()`, `MayBeModified()`, `AttributesModified()`, etc. Everything useful from `TDF_LabelNode` is accessible through the `TDF_Label` public facade.

3. **It's a deliberate internal class** — constructors, destructor, and operator new/delete are all private, accessible only to friend classes `TDF_Data` and `TDF_Label`.

This is different from the hypothetical "can't construct but navigate via pointers" scenario. There's no pointer to navigate to — it's truly internal. The exclude is correct as-is.

That said, let me check if there are other excluded headers where the "non-constructible but useful" pattern actually applies:

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- 🔄 Check for other excluded non-constructible classes
- ⬜ Implement non-constructible class support
- ⬜ Test and validate changes
- ⬜ Commit changes

Check other excluded non-constructible headers

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx">Message_LazyProgressScope.hxx</a></summary>

```
// Copyright (c) 2017-2021 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _Message_LazyProgressScope_HeaderFiler
#define _Message_LazyProgressScope_HeaderFiler

#include <Message_ProgressScope.hxx>

//! Progress scope with lazy updates and abort fetches.
//!
//! Although Message_ProgressIndicator implementation is encouraged to spare GUI updates,
//! even optimized implementation might show a noticeable overhead on a very small update step (e.g.
//! per triangle).
//!
//! The class splits initial (displayed) number of overall steps into larger chunks specified in
//! constructor, so that displayed progress is updated at larger steps.
class Message_LazyProgressScope : protected Message_ProgressScope
{
public:
  //! Main constructor.
  //! @param[in] theRange  progress range to scope
  //! @param[in] theName   name of this scope
  //! @param[in] theMax    number of steps within this scope
  //! @param[in] thePatchStep  number of steps to update progress
  //! @param[in] theIsInf  infinite flag
  Message_LazyProgressScope(const Message_ProgressRange& theRange,
                            const char*                  theName,
                            const Standard_Real          theMax,
                            const Standard_Real          thePatchStep,
                            const Standard_Boolean       theIsInf = Standard_False)
      : Message_ProgressScope(theRange, theName, theMax, theIsInf),
        myPatchStep(thePatchStep),
        myPatchProgress(0.0),
        myIsLazyAborted(Standard_False)
  {
  }

  //! Increment progress with 1.
  void Next()
  {
    if (++myPatchProgress < myPatchStep)
    {
      return;
    }

    myPatchProgress = 0.0;
    Message_ProgressScope::Next(myPatchStep);
    IsAborted();
  }

  //! Return TRUE if progress has been aborted - return the cached state lazily updated.
  Standard_Boolean More() const { return !myIsLazyAborted; }

  //! Return TRUE if progress has been aborted - fetches actual value from the Progress.
  Standard_Boolean IsAborted()
  {
    myIsLazyAborted = myIsLazyAborted || !Message_ProgressScope::More();
    return myIsLazyAborted;
  }

protected:
  Standard_Real    myPatchStep;
  Standard_Real    myPatchProgress;
  Standard_Boolean myIsLazyAborted;
};

#endif // _Message_LazyProgressScope_HeaderFiler

```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx">Poly_CoherentTriPtr.hxx</a></summary>

```
// Created on: 2007-12-08
// Created by: Alexander GRIGORIEV
// Copyright (c) 2007-2014 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef Poly_CoherentTriPtr_HeaderFile
#define Poly_CoherentTriPtr_HeaderFile

#include <NCollection_BaseAllocator.hxx>
#include <NCollection_DefineAlloc.hxx>

class Poly_CoherentTriangle;

#ifdef _MSC_VER
  #pragma warning(push)
  #pragma warning(disable : 4355) //'this' : used in base member initializer list
#endif

/**
 * Implementation of both list node for Poly_CoherentTriangle type and
 * round double-linked list of these nodes.
 */

class Poly_CoherentTriPtr
{
public:
  /**
   * Iterator class for this list of triangles. Because the list is round,
   * an iteration can be started from any member and it finishes before taking
   * this member again. The iteration sense is always forward (Next).
   */
  class Iterator
  {
  public:
    //! Empty constructor
    inline Iterator()
        : myFirst(0L),
          myCurrent(0L)
    {
    }

    //! Constructor
    inline Iterator(const Poly_CoherentTriPtr& thePtr)
        : myFirst(&thePtr),
          myCurrent(&thePtr)
    {
    }

    //! Query the triangle that started the current iteration.
    inline const Poly_CoherentTriangle* First() const
    {
      return myFirst ? &myFirst->GetTriangle() : 0L;
    }

    //! Query if there is available triangle pointer on this iteration
    inline Standard_Boolean More() const { return myCurrent != 0L; }

    //! Go to the next iteration.
    Standard_EXPORT void Next();

    //! Get the current iterated triangle
    inline const Poly_CoherentTriangle& Value() const { return myCurrent->GetTriangle(); }

    //! Get the current iterated triangle (mutable)
    inline Poly_CoherentTriangle& ChangeValue() const
    {
      return const_cast<Poly_CoherentTriangle&>(myCurrent->GetTriangle());
    }

    //! Get the current iterated pointer to triangle
    inline const Poly_CoherentTriPtr& PtrValue() const { return *myCurrent; }

  private:
    const Poly_CoherentTriPtr* myFirst;
    const Poly_CoherentTriPtr* myCurrent;
  };

  // ---------- PUBLIC METHODS ----------

  /**
   * Constructor.
   */
  inline Poly_CoherentTriPtr(const Poly_CoherentTriangle& theTri)
      : mypTriangle(&theTri),
        myNext(this),
        myPrevious(this)
  {
  }

  /**
   * Operator new for dynamic allocations
   */
  DEFINE_NCOLLECTION_ALLOC

  /**
   * Query the stored pointer to Triangle.
   */
  inline const Poly_CoherentTriangle& GetTriangle() const { return *mypTriangle; }

  /**
   * Initialize this instance with a pointer to triangle.
   */
  inline void SetTriangle(const Poly_CoherentTriangle* pTri) { mypTriangle = pTri; }

  /**
   * Query the next pointer in the list.
   */
  inline Poly_CoherentTriPtr& Next() const { return *myNext; }

  /**
   * Query the previous pointer in the list.
   */
  inline Poly_CoherentTriPtr& Previous() const { return *myPrevious; }

  /**
   * Append a pointer to triangle into the list after the current instance.
   * @param pTri
   *   Triangle that is to be included in the list after this one.
   * @param theA
   *   Allocator where the new pointer instance is created.
   */
  Standard_EXPORT void Append(const Poly_CoherentTriangle*             pTri,
                              const Handle(NCollection_BaseAllocator)& theA);

  /**
   * Prepend a pointer to triangle into the list before the current instance.
   * @par
... (truncated, 5442 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx">TopoDS_AlertAttribute.hxx</a></summary>

```
// Created on: 2018-06-10
// Created by: Natalia Ermolaeva
// Copyright (c) 2018-2020 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _TopoDS_AlertAttribute_HeaderFile
#define _TopoDS_AlertAttribute_HeaderFile

#include <Message_AttributeStream.hxx>
#include <Message_Messenger.hxx>
#include <Message_Report.hxx>

#include <TopoDS_Shape.hxx>

class Message_Messenger;

//! Alert attribute object storing TopoDS shape in its field
class TopoDS_AlertAttribute : public Message_AttributeStream
{
  DEFINE_STANDARD_RTTIEXT(TopoDS_AlertAttribute, Message_AttributeStream)
public:
  //! Constructor with shape argument
  Standard_EXPORT TopoDS_AlertAttribute(
    const TopoDS_Shape&            theShape,
    const TCollection_AsciiString& theName = TCollection_AsciiString());

  //! Returns contained shape
  const TopoDS_Shape& GetShape() const { return myShape; }

public:
  //! Push shape information into messenger
  Standard_EXPORT static void Send(const Handle(Message_Messenger)& theMessenger,
                                   const TopoDS_Shape&              theShape);

  //! Dumps the content of me into the stream
  Standard_EXPORT void DumpJson(Standard_OStream& theOStream,
                                Standard_Integer  theDepth = -1) const Standard_OVERRIDE;

private:
  TopoDS_Shape myShape;
};

inline const Handle(Message_Messenger)& operator<<(const Handle(Message_Messenger)& theMessenger,
                                                   const TopoDS_Shape&              theShape)
{
  TopoDS_AlertAttribute::Send(theMessenger, theShape);
  return theMessenger;
}

#endif // _TopoDS_AlertAttribute_HeaderFile

```
</details>

<details>
<summary>Searched for regex <code>construct|destruct|operator delete|operator new|non.?constructible</code> (<code>**/crates/opencascade-binding-generator/src/codegen/**</code>), 20 results</summary>

- [cpp.rs:4](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L4)
- [cpp.rs:21](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L21)
- [cpp.rs:39](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L39)
- [cpp.rs:278](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L278)
- [cpp.rs:279](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L279)
- [cpp.rs:283](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L283)
- [cpp.rs:293](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L293)
- [cpp.rs:302](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L302)
- [cpp.rs:305](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L305)
- [cpp.rs:309](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L309)
- [cpp.rs:361](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L361)
- [rust.rs:26](../crates/opencascade-binding-generator/src/codegen/rust.rs#L26)
- [rust.rs:38](../crates/opencascade-binding-generator/src/codegen/rust.rs#L38)
- [rust.rs:42](../crates/opencascade-binding-generator/src/codegen/rust.rs#L42)
- [rust.rs:43](../crates/opencascade-binding-generator/src/codegen/rust.rs#L43)
- [rust.rs:138](../crates/opencascade-binding-generator/src/codegen/rust.rs#L138)
- [rust.rs:139](../crates/opencascade-binding-generator/src/codegen/rust.rs#L139)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)
- [rust.rs:170](../crates/opencascade-binding-generator/src/codegen/rust.rs#L170)
</details>

<details>
<summary>Searched for files matching <code>**/crates/opencascade-binding-generator/src/codegen/*.rs</code>, 5 matches</summary>

- [collections.rs](../crates/opencascade-binding-generator/src/codegen/collections.rs)
- [bindings.rs](../crates/opencascade-binding-generator/src/codegen/bindings.rs)
- [rust.rs](../crates/opencascade-binding-generator/src/codegen/rust.rs)
- [mod.rs](../crates/opencascade-binding-generator/src/codegen/mod.rs)
- [cpp.rs](../crates/opencascade-binding-generator/src/codegen/cpp.rs)
</details>

<details>
<summary>Searched for text <code>Message_LazyProgressScope</code> (<code>**/crates/occt-sys/OCCT/src/**</code>), 20 results</summary>

- [RWObj_CafWriter.cxx:17](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L17)
- [RWObj_CafWriter.cxx:184](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L184)
- [RWObj_CafWriter.cxx:264](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L264)
- [RWObj_CafWriter.cxx:340](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L340)
- [RWObj_CafWriter.cxx:361](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L361)
- [RWObj_CafWriter.cxx:383](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L383)
- [RWObj_CafWriter.cxx:403](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.cxx#L403)
- [RWObj_CafWriter.hxx:30](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L30)
- [RWObj_CafWriter.hxx:118](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L118)
- [RWObj_CafWriter.hxx:130](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L130)
- [RWObj_CafWriter.hxx:139](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L139)
- [RWObj_CafWriter.hxx:148](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L148)
- [RWObj_CafWriter.hxx:157](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L157)
- [RWPly_CafWriter.hxx:30](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L30)
- [RWPly_CafWriter.hxx:159](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L159)
- [RWPly_CafWriter.hxx:171](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L171)
- [RWPly_CafWriter.hxx:180](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L180)
- [RWPly_CafWriter.cxx:17](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.cxx#L17)
- [RWPly_CafWriter.cxx:142](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.cxx#L142)
- [RWPly_CafWriter.cxx:199](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.cxx#L199)
</details>

<details>
<summary>Searched for text <code>Poly_CoherentTriPtr</code> (<code>**/crates/occt-sys/OCCT/src/**</code>), 20 results</summary>

- [FILES:16](../crates/occt-sys/OCCT/src/Poly/FILES#L16)
- [FILES:17](../crates/occt-sys/OCCT/src/Poly/FILES#L17)
- [Poly_CoherentNode.cxx:23](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L23)
- [Poly_CoherentNode.cxx:50](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L50)
- [Poly_CoherentNode.cxx:64](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L64)
- [Poly_CoherentNode.cxx:69](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L69)
- [Poly_CoherentNode.cxx:74](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L74)
- [Poly_CoherentNode.cxx:78](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L78)
- [Poly_CoherentNode.cxx:93](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.cxx#L93)
- [Poly_CoherentTriPtr.hxx:16](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L16)
- [Poly_CoherentTriPtr.hxx:17](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L17)
- [Poly_CoherentTriPtr.hxx:34](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L34)
- [Poly_CoherentTriPtr.hxx:53](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L53)
- [Poly_CoherentTriPtr.hxx:81](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L81)
- [Poly_CoherentTriPtr.hxx:84](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L84)
- [Poly_CoherentTriPtr.hxx:85](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L85)
- [Poly_CoherentTriPtr.hxx:93](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L93)
- [Poly_CoherentTriPtr.hxx:118](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L118)
- [Poly_CoherentTriPtr.hxx:123](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L123)
</details>

<details>
<summary>Searched for text <code>TopoDS_AlertAttribute</code> (<code>**/crates/occt-sys/OCCT/src/**</code>), 16 results</summary>

- [TopoDS_AlertAttribute.cxx:16](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.cxx#L16)
- [TopoDS_AlertAttribute.cxx:20](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.cxx#L20)
- [TopoDS_AlertAttribute.cxx:24](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.cxx#L24)
- [TopoDS_AlertAttribute.cxx:37](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.cxx#L37)
- [TopoDS_AlertAttribute.cxx:54](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.cxx#L54)
- [TopoDS_AlertAttribute.cxx:60](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.cxx#L60)
- [FILES:2](../crates/occt-sys/OCCT/src/TopoDS/FILES#L2)
- [FILES:3](../crates/occt-sys/OCCT/src/TopoDS/FILES#L3)
- [TopoDS_AlertAttribute.hxx:16](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L16)
- [TopoDS_AlertAttribute.hxx:17](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L17)
- [TopoDS_AlertAttribute.hxx:28](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L28)
- [TopoDS_AlertAttribute.hxx:30](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L30)
- [TopoDS_AlertAttribute.hxx:33](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L33)
- [TopoDS_AlertAttribute.hxx:56](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L56)
- [TopoDS_AlertAttribute.hxx:60](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L60)
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs">cpp.rs</a></summary>

```
//! C++ wrapper code generation
//!
//! Generates extern "C" wrapper functions for all OCCT methods:
//! - Constructors (using placement new or heap allocation)
//! - Return-by-value methods
//! - Static methods
//! - Overloaded methods
//!
//! All wrapper functions use extern "C" linkage for direct FFI access.

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::HashSet;
use std::fmt::Write;

fn collect_handle_types(classes: &[&ParsedClass]) -> Vec<(String, String)> {
    let mut handles = HashSet::new();

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
        }

        for method in &class.methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for method in &class.static_methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }
    }

    let mut result: Vec<_> = handles
        .into_iter()
        .filter(|inner_class| {
            // Skip namespace-scoped types (e.g., IMeshData::ListOfPnt2d) and pointer types
            !inner_class.contains("::") && !inner_class.contains('*') && !inner_class.contains('&')
        })
        .map(|inner_class| {
            // Use full class name to match Rust side (e.g., HandleGeom2dCurve not HandleCurve)
            let handle_name = format!("Handle{}", inner_class.replace("_", ""));
            (inner_class, handle_name)
        })
        .collect();
    result.sort();
    result
}


/// Collect Handle type inner classes from a type
fn collect_type_handles(ty: &Option<Type>, handles: &mut HashSet<String>) {
    if let Some(ty) = ty {
        match ty {
            Type::Handle(name) => {
                handles.insert(name.clone());
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                collect_type_handles(&Some(inner.as_ref().clone()), handles);
            }
            _ => {}
        }
    }
}

/// Collect headers needed for a type
fn collect_type_headers(ty: &Option<Type>, headers: &mut HashSet<String>, known_headers: &HashSet<String>) {
    if let Some(ty) = ty {
        // Skip unbindable types (arrays, streams, void pointers, raw pointers, etc.)
        if ty.is_unbindable() {
            return;
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address"
                ) {
                    return;
                }
                // For nested types (Parent::Nested), include the parent class header
                if let Some(parent) = name.split("::").next() {
                    if name.contains("::") {
                        // Nested type — include the parent's header
                        if parent.contains('_') || parent.starts_with("Standard") {
                            let header = format!("{}.hxx", parent);
                            if known_headers.is_empty() || known_headers.contains(&header) {
                                headers.insert(header);
                            }
   
... (truncated, 14009 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs">rust.rs</a></summary>

```
//! Rust extern "C" FFI code generation
//!
//! Generates the extern "C" FFI module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, Type};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// Generate source attribution for a declaration (header, line number, and C++ identifier)
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
    }
}

/// Types collected from class interfaces
pub struct CollectedTypes {
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve") - sorted for deterministic output
    pub classes: BTreeSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>) - sorted for deterministic output
    pub handles: BTreeSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: BTreeSet::new(),
        handles: BTreeSet::new(),
    };

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // This ensures the Handle type is declared even if not used in method signatures
        if class.is_handle_type && !class.has_protected_destructor {
            result.handles.insert(class.name.clone());
        }

        // From constructors
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_types_from_type(&param.ty, &mut result);
            }
        }

        // From methods
        for method in &class.methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }

        // From static methods
        for method in &class.static_methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, streams, void ptrs, etc.)
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types that may come from canonical type resolution
            if !is_primitive_type(name) {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            collected.handles.insert(name.clone());
            collected.classes.insert(name.clone());
        }
        Type::ConstRef(inner)
        | Type::MutRef(inner)
        | Type::ConstPtr(inner)
        | Type::MutPtr(inner) => {
            collect_types_from_type(inner, collected);
        }
        _ => {}
    }
}

/// Check if a type name is a primitive (not an OCCT class)
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
        "c_long" | "c_ulong" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned long long" | "short" | "unsigned short" |
        "signed char" | "unsigned char"
    )
}

// ========
... (truncated, 51903 chars)
```
</details>

<details>
<summary>Reading all lines of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs">bindings.rs</a></summary>

```
//! Shared intermediate representation for binding decisions.
//!
//! `ClassBindings` computes all filtering, naming, overload suffixes,
//! and conflict resolution for a class **once**. The emit functions for
//! ffi.rs, wrappers.hxx, and per-module re-exports consume this struct
//! without re-deriving any decisions.

use crate::model::{Constructor, Method, Param, ParsedClass, ParsedField, StaticMethod, Type};
use crate::module_graph;
use crate::parser;
use crate::resolver::{self, SymbolTable};
use crate::type_mapping::{self, map_return_type_in_context, map_type_in_context, TypeContext};
use heck::ToSnakeCase;
use std::fmt::Write as _;
use std::collections::{HashMap, HashSet};

/// Rust keywords that need suffix escaping (FFI doesn't support raw identifiers).
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

// ── IR Structs ──────────────────────────────────────────────────────────────

/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
#[derive(Debug, Clone)]
pub struct ClassBindings {
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub is_handle_type: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,
    pub wrapper_methods: Vec<WrapperMethodBinding>,
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
    pub has_to_handle: bool,
    /// Whether Handle_get/get_mut should be generated (true for all handle types, including abstract)
    pub has_handle_get: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub handle_downcasts: Vec<HandleDowncastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
    /// Whether this class is a POD struct that can be represented with real fields
    pub is_pod_struct: bool,
    /// Fields for POD structs (only populated when is_pod_struct is true)
    pub pod_fields: Vec<PodFieldBinding>,
}

/// A single field in a POD struct.
#[derive(Debug, Clone)]
pub struct PodFieldBinding {
    /// Field name in Rust (snake_case)
    pub rust_name: String,
    /// Field name in C++ (original)
    pub cpp_name: String,
    /// Rust type string, e.g. "bool" or "f64"
    pub rust_type: String,
    /// Array size if this is a fixed-size array field
    pub array_size: Option<usize>,
    /// Byte offset for offsetof check
    pub offset_index: usize,
    /// Doc comment
    pub doc_comment: Option<String>,
}

/// A constructor that will have a C++ wrapper (std::make_unique),
/// or a Rust-only convenience wrapper that delegates to a full-argument constructor.
#[derive(Debug, Clone)]
pub struct ConstructorBinding {
    /// FFI function name, e.g. "gp_Pnt_ctor_real3"
    pub ffi_fn_name: String,
    /// Impl method name in re-export, e.g. "new_real3"
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// C++ argument expressions for calling the constructor
    pub cpp_arg_exprs: Vec<String>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// If this is a convenience wrapper (fewer params
... (truncated, 206778 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01HgeSJ2CapSaGy88rBZBtjw__vscode-1771312782228/content.txt)

<details>
<summary>Searched for regex <code>Poly_CoherentTriPtr</code> (<code>**/crates/occt-sys/OCCT/src/**/*.hxx</code>), 18 results</summary>

- [Poly_CoherentTriPtr.hxx:16](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L16)
- [Poly_CoherentTriPtr.hxx:17](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L17)
- [Poly_CoherentTriPtr.hxx:34](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L34)
- [Poly_CoherentTriPtr.hxx:53](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L53)
- [Poly_CoherentTriPtr.hxx:81](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L81)
- [Poly_CoherentTriPtr.hxx:84](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L84)
- [Poly_CoherentTriPtr.hxx:85](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L85)
- [Poly_CoherentTriPtr.hxx:93](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L93)
- [Poly_CoherentTriPtr.hxx:118](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L118)
- [Poly_CoherentTriPtr.hxx:123](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L123)
- [Poly_CoherentTriPtr.hxx:152](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L152)
- [Poly_CoherentTriPtr.hxx:158](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L158)
- [Poly_CoherentTriPtr.hxx:167](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L167)
- [Poly_CoherentTriPtr.hxx:178](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L178)
- [Poly_CoherentTriPtr.hxx:179](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L179)
- [Poly_CoherentNode.hxx:20](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L20)
- [Poly_CoherentNode.hxx:142](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L142)
- [Poly_CoherentNode.hxx:158](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L158)
</details>

<details>
<summary>Searched for text <code>Message_LazyProgressScope</code> (<code>**/crates/occt-sys/OCCT/src/**/*.hxx</code>), 15 results</summary>

- [RWObj_CafWriter.hxx:30](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L30)
- [RWObj_CafWriter.hxx:118](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L118)
- [RWObj_CafWriter.hxx:130](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L130)
- [RWObj_CafWriter.hxx:139](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L139)
- [RWObj_CafWriter.hxx:148](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L148)
- [RWObj_CafWriter.hxx:157](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L157)
- [RWPly_CafWriter.hxx:30](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L30)
- [RWPly_CafWriter.hxx:159](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L159)
- [RWPly_CafWriter.hxx:171](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L171)
- [RWPly_CafWriter.hxx:180](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L180)
- [Message_LazyProgressScope.hxx:14](../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx#L14)
- [Message_LazyProgressScope.hxx:15](../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx#L15)
- [Message_LazyProgressScope.hxx:27](../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx#L27)
- [Message_LazyProgressScope.hxx:36](../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx#L36)
- [Message_LazyProgressScope.hxx:77](../crates/occt-sys/OCCT/src/Message/Message_LazyProgressScope.hxx#L77)
</details>

<details>
<summary>Searched for regex <code>public|protected|private|Message_LazyProgressScope</code> (<code>**/crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx</code>), 11 results</summary>

- [RWObj_CafWriter.hxx:6](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L6)
- [RWObj_CafWriter.hxx:30](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L30)
- [RWObj_CafWriter.hxx:35](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L35)
- [RWObj_CafWriter.hxx:38](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L38)
- [RWObj_CafWriter.hxx:92](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L92)
- [RWObj_CafWriter.hxx:118](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L118)
- [RWObj_CafWriter.hxx:130](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L130)
- [RWObj_CafWriter.hxx:139](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L139)
- [RWObj_CafWriter.hxx:148](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L148)
- [RWObj_CafWriter.hxx:157](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L157)
- [RWObj_CafWriter.hxx:160](../crates/occt-sys/OCCT/src/RWObj/RWObj_CafWriter.hxx#L160)
</details>

<details>
<summary>Searched for regex <code>public|protected|private|Message_LazyProgressScope</code> (<code>**/crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx</code>), 11 results</summary>

- [RWPly_CafWriter.hxx:6](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L6)
- [RWPly_CafWriter.hxx:30](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L30)
- [RWPly_CafWriter.hxx:34](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L34)
- [RWPly_CafWriter.hxx:37](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L37)
- [RWPly_CafWriter.hxx:63](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L63)
- [RWPly_CafWriter.hxx:111](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L111)
- [RWPly_CafWriter.hxx:139](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L139)
- [RWPly_CafWriter.hxx:159](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L159)
- [RWPly_CafWriter.hxx:171](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L171)
- [RWPly_CafWriter.hxx:180](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L180)
- [RWPly_CafWriter.hxx:183](../crates/occt-sys/OCCT/src/RWPly/RWPly_CafWriter.hxx#L183)
</details>

<details>
<summary>Searched for regex <code>constructor|destructor|protected|non.?construct|placement.?new|has_protected</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:8](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L8)
- [bindings.rs:40](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L40)
- [bindings.rs:45](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L45)
- [bindings.rs:80](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L80)
- [bindings.rs:81](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L81)
- [bindings.rs:83](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L83)
- [bindings.rs:90](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L90)
- [bindings.rs:97](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L97)
- [bindings.rs:103](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L103)
- [bindings.rs:106](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L106)
- [bindings.rs:498](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L498)
- [bindings.rs:652](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L652)
- [bindings.rs:653](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L653)
- [bindings.rs:655](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L655)
- [bindings.rs:656](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L656)
- [bindings.rs:673](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L673)
- [bindings.rs:677](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L677)
- [bindings.rs:679](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L679)
</details>

<details>
<summary>Searched for regex <code>public|protected|private|Poly_CoherentTriPtr</code> (<code>**/crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx</code>), 11 results</summary>

- [Poly_CoherentNode.hxx:8](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L8)
- [Poly_CoherentNode.hxx:20](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L20)
- [Poly_CoherentNode.hxx:36](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L36)
- [Poly_CoherentNode.hxx:38](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L38)
- [Poly_CoherentNode.hxx:39](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L39)
- [Poly_CoherentNode.hxx:142](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L142)
- [Poly_CoherentNode.hxx:151](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L151)
- [Poly_CoherentNode.hxx:152](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L152)
- [Poly_CoherentNode.hxx:154](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L154)
- [Poly_CoherentNode.hxx:155](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L155)
- [Poly_CoherentNode.hxx:158](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentNode.hxx#L158)
</details>

<details>
<summary>Searched for regex <code>has_protected_destructor</code> (<code>**/crates/opencascade-binding-generator/src/**</code>), 20 results</summary>

- [parser.rs:406](../crates/opencascade-binding-generator/src/parser.rs#L406)
- [parser.rs:413](../crates/opencascade-binding-generator/src/parser.rs#L413)
- [parser.rs:587](../crates/opencascade-binding-generator/src/parser.rs#L587)
- [main.rs:465](../crates/opencascade-binding-generator/src/main.rs#L465)
- [main.rs:649](../crates/opencascade-binding-generator/src/main.rs#L649)
- [main.rs:672](../crates/opencascade-binding-generator/src/main.rs#L672)
- [model.rs:132](../crates/opencascade-binding-generator/src/model.rs#L132)
- [resolver.rs:124](../crates/opencascade-binding-generator/src/resolver.rs#L124)
- [resolver.rs:353](../crates/opencascade-binding-generator/src/resolver.rs#L353)
- [resolver.rs:470](../crates/opencascade-binding-generator/src/resolver.rs#L470)
- [resolver.rs:779](../crates/opencascade-binding-generator/src/resolver.rs#L779)
- [resolver.rs:950](../crates/opencascade-binding-generator/src/resolver.rs#L950)
- [resolver.rs:1017](../crates/opencascade-binding-generator/src/resolver.rs#L1017)
- [rust.rs:38](../crates/opencascade-binding-generator/src/codegen/rust.rs#L38)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)
- [rust.rs:401](../crates/opencascade-binding-generator/src/codegen/rust.rs#L401)
- [rust.rs:794](../crates/opencascade-binding-generator/src/codegen/rust.rs#L794)
- [rust.rs:808](../crates/opencascade-binding-generator/src/codegen/rust.rs#L808)
- [rust.rs:822](../crates/opencascade-binding-generator/src/codegen/rust.rs#L822)
</details>

Searched for regex `DEFINE_NCOLLECTION_ALLOC|operator delete|non.?constructible` (`**/crates/opencascade-binding-generator/src/**`), no results

<details>
<summary>Reading lines 390-600 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L390-L601">parser.rs</a></summary>

```

    // Skip internal node types that use custom allocators (can't be used with std::unique_ptr)
    if name.ends_with("Node") && name.starts_with("NCollection_") {
        if verbose {
            println!("    Skipping {} (internal node type)", name);
        }
        return Vec::new();
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    // Extract direct base classes for upcast generation
    let base_classes = extract_base_classes(entity);
    
    // Check for protected/private destructor (indicates non-instantiable abstract base class)
    let has_protected_destructor = check_protected_destructor(entity);

    if verbose {
        println!("  Parsing class: {}", name);
        if !base_classes.is_empty() {
            println!("    Base classes: {:?}", base_classes);
        }
        if has_protected_destructor {
            println!("    Has protected destructor (non-instantiable)");
        }
    }

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut static_methods = Vec::new();
    let mut fields: Vec<ParsedField> = Vec::new();
    let mut has_non_public_fields = false;
    let mut has_virtual_methods = false;
    let mut all_method_names = std::collections::HashSet::new();
    let mut is_abstract = false;
    let mut pure_virtual_methods = std::collections::HashSet::new();
    let mut has_explicit_constructors = false;
    let mut nested_classes: Vec<ParsedClass> = Vec::new();

    // Track current access level for nested type visibility.
    // Default: `class` => private, `struct` => public.
    let default_access = if entity.get_kind() == EntityKind::StructDecl {
        Accessibility::Public
    } else {
        Accessibility::Private
    };
    let current_access = std::cell::Cell::new(default_access);

    // Check if there's a DEFINE_STANDARD_HANDLE for this class
    // This is typically done outside the class, so we check the name pattern
    // and look for inheritance from Standard_Transient
    let is_handle_type = check_is_handle_type(entity);

    entity.visit_children(|child, _| {
        // Track access specifiers (public:/protected:/private: sections)
        if child.get_kind() == EntityKind::AccessSpecifier {
            if let Some(acc) = child.get_accessibility() {
                current_access.set(acc);
            }
            return EntityVisitResult::Continue;
        }

        match child.get_kind() {
            EntityKind::Constructor => {
                // Any explicit constructor means C++ won't generate an implicit default
                has_explicit_constructors = true;

                // Skip deprecated constructors
                if child.get_availability() == Availability::Deprecated {
                    if verbose {
                        println!("    Skipping deprecated constructor for {}", name);
                    }
                    return EntityVisitResult::Continue;
                }

                if is_public(&child) {
                    if let Some(ctor) = parse_constructor(&child, verbose) {
                        constructors.push(ctor);
                    }
                }
            }
            EntityKind::Method => {
                // Check for virtual methods (affects POD detection)
                if child.is_virtual_method() {
                    has_virtual_methods = true;
                }
                // Check if this is a pure virtual method (makes the class abstract)
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                    if let Some(ref method_name) = child.get_name() {
                        pure_virtual_methods.insert(method_name.clone());
                    }
                }

                // Skip destructors, operators, and conversion functions
                if let Some(ref method_name) = child.get_name() {
                    if method_name.starts_with('~')
   
... (truncated, 8370 chars)
```
</details>

<details>
<summary>Reading lines 25-115 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L25-L116">bindings.rs</a></summary>

```

// ── IR Structs ──────────────────────────────────────────────────────────────

/// Computed binding decisions for a single class.
/// All filtering, naming, and conflict resolution happens here ONCE.
#[derive(Debug, Clone)]
pub struct ClassBindings {
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub is_handle_type: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,
    pub wrapper_methods: Vec<WrapperMethodBinding>,
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
    pub has_to_handle: bool,
    /// Whether Handle_get/get_mut should be generated (true for all handle types, including abstract)
    pub has_handle_get: bool,
    pub handle_upcasts: Vec<HandleUpcastBinding>,
    pub handle_downcasts: Vec<HandleDowncastBinding>,
    pub inherited_methods: Vec<InheritedMethodBinding>,
    /// Whether this class is a POD struct that can be represented with real fields
    pub is_pod_struct: bool,
    /// Fields for POD structs (only populated when is_pod_struct is true)
    pub pod_fields: Vec<PodFieldBinding>,
}

/// A single field in a POD struct.
#[derive(Debug, Clone)]
pub struct PodFieldBinding {
    /// Field name in Rust (snake_case)
    pub rust_name: String,
    /// Field name in C++ (original)
    pub cpp_name: String,
    /// Rust type string, e.g. "bool" or "f64"
    pub rust_type: String,
    /// Array size if this is a fixed-size array field
    pub array_size: Option<usize>,
    /// Byte offset for offsetof check
    pub offset_index: usize,
    /// Doc comment
    pub doc_comment: Option<String>,
}

/// A constructor that will have a C++ wrapper (std::make_unique),
/// or a Rust-only convenience wrapper that delegates to a full-argument constructor.
#[derive(Debug, Clone)]
pub struct ConstructorBinding {
    /// FFI function name, e.g. "gp_Pnt_ctor_real3"
    pub ffi_fn_name: String,
    /// Impl method name in re-export, e.g. "new_real3"
    pub impl_method_name: String,
    /// Parameters
    pub params: Vec<ParamBinding>,
    /// C++ argument expressions for calling the constructor
    pub cpp_arg_exprs: Vec<String>,
    /// Doc comment
    pub doc_comment: Option<String>,
    /// Source line in C++ header
    pub source_line: Option<u32>,
    /// If this is a convenience wrapper (fewer params with defaults filled in),
    /// contains info about the full-argument constructor it delegates to.
    /// When set, no ffi.rs or wrappers.hxx entry is generated — only a Rust-only
    /// method in the module re-export that calls the full-argument version.
    pub convenience_of: Option<ConvenienceInfo>,
}

/// Info for a convenience constructor that delegates to a full-argument version.
#[derive(Debug, Clone)]
pub struct ConvenienceInfo {
    /// The impl_method_name of the full-argument constructor to call
    pub full_method_name: String,
    /// Rust expressions for the default values of the trimmed trailing params,
    /// in order. E.g. ["false", "false"] for two defaulted bool params.
    pub default_exprs: Vec<String>,
}

/// A method bound as a direct extern "C" wrapper (self receiver, no wrapper needed).
#[derive(Debug, Clone)]
pub struct DirectMethodBinding {
```
</details>

<details>
<summary>Reading lines 130-200 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L130-L201">rust.rs</a></summary>

```
pub fn generate_ffi(
    all_classes: &[&ParsedClass],
    all_headers: &[String],
    collections: &[super::collections::CollectionInfo],
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors
    let protected_destructor_class_names = symbol_table.protected_destructor_class_names();

    // All enum names (needed for opaque type filtering)
    let all_enum_names = &symbol_table.all_enum_names;

    // Collect collection type names to exclude from class generation
    // Collections are generated separately with specialized wrappers
    let collection_type_names: HashSet<String> = collections.iter()
        .map(|c| c.typedef_name.clone())
        .collect();

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: String = all_bindings
        .iter()
        .filter(|b| !b.has_protected_destructor)
        .filter(|b| !collection_type_names.contains(&b.cpp_name))
        .map(|b| super::bindings::emit_ffi_class(b))
        .collect();

    // Generate namespace-level free functions from pre-computed FunctionBindings
    let function_items = generate_functions_from_bindings(function_bindings);

    // Generate Handle type declarations
    let handle_decls = generate_handle_declarations(all_classes, &symbol_table.handle_able_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(all_classes);
    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
        &collection_type_names,
    );

    // Generate nested type destructor declarations for ffi extern block
    let nested_destructor_decls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "    // ========================").unwrap();
        writeln!(s, "    // Nested type destructors").unwrap();
        writeln!(s, "    // ========================").unwrap();
        writeln!(s).unwrap();
        for nt in &nested_types {
            writeln!(s, "    pub fn {}_destructor(self_: *mut {});", nt.ffi_name, nt.ffi_name).unwrap();
        }
        s
    };

    // Generate CppDeletable impls for nested types
    let nested_deletable_impls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "// CppDeletable impls for nested types").unwrap();
        for nt in &nested_types {
            writeln!(s, "unsafe impl crate::CppDeletable for {} {{", nt.ffi_name).unwrap();
            writeln!(s, "    unsafe fn cpp_delete(ptr: *mut Self) {{").unwrap();
            writeln!(s, "        {}_destructor(ptr);", nt.ffi_name).unwrap();
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_011uyX5oGdJdzPKtHV7ztstr__vscode-1771312782240/content.txt)

Searched for text `check_protected_destructor` (`**/crates/opencascade-binding-generator/src/parser.rs`), 2 results
- [parser.rs:406](../crates/opencascade-binding-generator/src/parser.rs#L406)
- [parser.rs:605](../crates/opencascade-binding-generator/src/parser.rs#L605)

<details>
<summary>Reading lines 605-660 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L605-L661">parser.rs</a></summary>

```
fn check_protected_destructor(entity: &Entity) -> bool {
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::Destructor {
            // Check if the destructor is not public
            if let Some(accessibility) = child.get_accessibility() {
                return accessibility != clang::Accessibility::Public;
            }
        }
    }
    false
}

/// Parse an enum declaration
fn parse_enum(entity: &Entity, source_header: &str, verbose: bool) -> Option<ParsedEnum> {
    let name = entity.get_name()?;

    // Skip anonymous enums (empty name or compiler-generated "(unnamed enum at ...)")
    if name.is_empty() || name.starts_with("(unnamed") {
        return None;
    }

    // Skip internal enums
    if name.starts_with('_') {
        return None;
    }

    // Skip nested enums (enums defined inside a class/struct)
    // These are not accessible at global scope
    if let Some(parent) = entity.get_semantic_parent() {
        let parent_kind = parent.get_kind();
        if parent_kind == EntityKind::ClassDecl || parent_kind == EntityKind::StructDecl {
            if verbose {
                println!("    Skipping {} (nested enum inside class)", name);
            }
            return None;
        }
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    if verbose {
        println!("  Parsing enum: {}", name);
    }

    let mut variants = Vec::new();

    entity.visit_children(|child, _| {
        if child.get_kind() == EntityKind::EnumConstantDecl {
            if let Some(variant_name) = child.get_name() {
                let value = child.get_enum_constant_value().map(|(signed, _unsigned)| signed);
                let comment = extract_doxygen_comment(&child);

                if verbose {
                    if let Some(v) = value {
                        println!("    Variant: {} = {}", variant_name, v);
```
</details>

<details>
<summary>Reading lines 380-450 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L380-L451">rust.rs</a></summary>

```
            .map(|rt| format!(" -> {}", rt.rust_ffi_type))
            .unwrap_or_default();

        let source_attr = format_source_attribution(
            &func.source_header,
            func.source_line,
            &format!("{}::{}", func.namespace, func.short_name),
        );
        writeln!(out, "    /// {}", source_attr).unwrap();
        writeln!(out, "    pub fn {}({}){};\n", func.cpp_wrapper_name, params_str, ret_str).unwrap();
    }
    out
}

/// Generate Handle type declarations
fn generate_handle_declarations(classes: &[&ParsedClass], extra_handle_able: &HashSet<String>) -> String {
    let mut handles = BTreeSet::new();

    // Classes parsed from non-excluded headers
    let mut defined_handles = BTreeSet::new();
    for class in classes {
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
            defined_handles.insert(class.name.clone());
        }
    }

    // Also generate Handle declarations for types that appear in Handle(...)
    // in method signatures, even if their own headers are excluded.
    // This ensures that methods like GeomAPI_Interpolate(Handle(TColgp_HArray1OfPnt))
    // can be generated even when TColgp_HArray1OfPnt.hxx is excluded.
    for name in extra_handle_able {
        handles.insert(name.clone());
    }

    let mut out = String::new();
    for class_name in &handles {
        let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
        writeln!(out, "/// Handle to {}", class_name).unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", handle_type_name).unwrap();
    }

    // For extra handle types (not from parsed classes), generate standalone
    // CppDeletable impls and destructor FFI declarations.
    // Parsed classes get these in their module files instead.
    let extra_handles: Vec<_> = handles.iter()
        .filter(|name| !defined_handles.contains(*name))
        .collect();
    if !extra_handles.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "extern \"C\" {{").unwrap();
        for class_name in &extra_handles {
            let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
            writeln!(out, "    pub fn {}_destructor(ptr: *mut {});", handle_type_name, handle_type_name).unwrap();
        }
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
        for class_name in &extra_handles {
            let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
            writeln!(out, "unsafe impl crate::CppDeletable for {} {{", handle_type_name).unwrap();
            writeln!(out, "    unsafe fn cpp_delete(ptr: *mut Self) {{").unwrap();
            writeln!(out, "        {}_destructor(ptr);", handle_type_name).unwrap();
            writeln!(out, "    }}").unwrap();
            writeln!(out, "}}").unwrap();
        }
    }

    out
}

```
</details>

<details>
<summary>Reading lines 780-860 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L780-L861">rust.rs</a></summary>

```
    if !collections.is_empty() {
        output.push('\n');
    }

    // Generate impl blocks for collection types
    for coll in collections {
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
                base_handle_reexports.insert(hd.derived_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Handle type re-exports (targets of handle upcasts/downcasts)\n");
        for handle_name in &base_handle_reexports {
            output.push_str(&format!("pub use crate::ffi::{};\n", handle_name));
        }
        output.push_str("\n");
    }

    for (header, header_bindings) in bindings_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");

        for bindings in header_bindings {
            output.push_str(&super::bindings::emit_reexport_class(bindings, module_name));
        }
    }

    // Re-export additional types (handles, opaque references, collection iterators)
    // that appear in ffi.rs but aren't covered by ClassBindings or collections.
    // Skip types already re-exported by ClassBindings (directly_exported_handles or base_handle_reexports).
    if !extra_types.is_empty() {
```
</details>

<details>
<summary>Searched for text <code>DEFINE_NCOLLECTION_ALLOC</code> (<code>**/crates/occt-sys/OCCT/src/**</code>), 5 results</summary>

- [TDF_LabelNode.hxx:103](../crates/occt-sys/OCCT/src/TDF/TDF_LabelNode.hxx#L103)
- [OpenGl_ShadowMap.hxx:109](../crates/occt-sys/OCCT/src/OpenGl/OpenGl_ShadowMap.hxx#L109)
- [Poly_CoherentTriPtr.hxx:103](../crates/occt-sys/OCCT/src/Poly/Poly_CoherentTriPtr.hxx#L103)
- [IMeshData_Types.hxx:55](../crates/occt-sys/OCCT/src/IMeshData/IMeshData_Types.hxx#L55)
- [NCollection_DynamicArray.hxx:61](../crates/occt-sys/OCCT/src/NCollection/NCollection_DynamicArray.hxx#L61)
</details>

<details>
<summary>Searched for regex <code>Message_LazyProgressScope|Poly_CoherentTriPtr|TopoDS_AlertAttribute|TDF_LabelNode</code> (<code>**/crates/opencascade-sys/bindings.toml</code>), 7 results</summary>

- [bindings.toml:229](../crates/opencascade-sys/bindings.toml#L229)
- [bindings.toml:231](../crates/opencascade-sys/bindings.toml#L231)
- [bindings.toml:232](../crates/opencascade-sys/bindings.toml#L232)
- [bindings.toml:234](../crates/opencascade-sys/bindings.toml#L234)
- [bindings.toml:235](../crates/opencascade-sys/bindings.toml#L235)
- [bindings.toml:248](../crates/opencascade-sys/bindings.toml#L248)
- [bindings.toml:249](../crates/opencascade-sys/bindings.toml#L249)
</details>

<details>
<summary>Searched for regex <code>define DEFINE_NCOLLECTION_ALLOC|operator new|operator delete</code> (<code>**/crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx</code>), 5 results</summary>

- [NCollection_DefineAlloc.hxx:25](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L25)
- [NCollection_DefineAlloc.hxx:26](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L26)
- [NCollection_DefineAlloc.hxx:31](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L31)
- [NCollection_DefineAlloc.hxx:32](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L32)
- [NCollection_DefineAlloc.hxx:36](../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L36)
</details>

<details>
<summary>Reading lines 20-50 of <a href="../crates/occt-sys/OCCT/src/NCollection/NCollection_DefineAlloc.hxx#L20-L51">NCollection_DefineAlloc.hxx</a></summary>

```

// Macro to overload placement new and delete operators for NCollection allocators.
// For Borland C and old SUN compilers do not define placement delete
// as it is not supported.
#if defined(__BORLANDC__) || (defined(__SUNPRO_CC) && (__SUNPRO_CC <= 0x530))
  #define DEFINE_NCOLLECTION_ALLOC                                                                 \
    void* operator new(size_t theSize, const Handle(NCollection_BaseAllocator)& theAllocator)      \
    {                                                                                              \
      return theAllocator->Allocate(theSize);                                                      \
    }
#else
  #define DEFINE_NCOLLECTION_ALLOC                                                                 \
    void* operator new(size_t theSize, const Handle(NCollection_BaseAllocator)& theAllocator)      \
    {                                                                                              \
      return theAllocator->Allocate(theSize);                                                      \
    }                                                                                              \
    void operator delete(void* theAddress, const Handle(NCollection_BaseAllocator)& theAllocator)  \
    {                                                                                              \
      theAllocator->Free(theAddress);                                                              \
    }
#endif

#endif

```
</details>

<details>
<summary>Searched for regex <code>delete|copy|Message_AttributeStream</code> (<code>**/crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx</code>), 7 results</summary>

- [Message_AttributeStream.hxx:1](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L1)
- [Message_AttributeStream.hxx:14](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L14)
- [Message_AttributeStream.hxx:15](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L15)
- [Message_AttributeStream.hxx:22](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L22)
- [Message_AttributeStream.hxx:24](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L24)
- [Message_AttributeStream.hxx:27](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L27)
- [Message_AttributeStream.hxx:45](../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L45)
</details>

<details>
<summary>Searched for text <code>TopoDS_AlertAttribute</code> (<code>**/crates/occt-sys/OCCT/src/**/*.hxx</code>), 7 results</summary>

- [TopoDS_AlertAttribute.hxx:16](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L16)
- [TopoDS_AlertAttribute.hxx:17](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L17)
- [TopoDS_AlertAttribute.hxx:28](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L28)
- [TopoDS_AlertAttribute.hxx:30](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L30)
- [TopoDS_AlertAttribute.hxx:33](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L33)
- [TopoDS_AlertAttribute.hxx:56](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L56)
- [TopoDS_AlertAttribute.hxx:60](../crates/occt-sys/OCCT/src/TopoDS/TopoDS_AlertAttribute.hxx#L60)
</details>

<details>
<summary>Reading lines 1-50 of <a href="../crates/occt-sys/OCCT/src/Message/Message_AttributeStream.hxx#L1-L51">Message_AttributeStream.hxx</a></summary>

```
// Copyright (c) 2020 OPEN CASCADE SAS
//
// This file is part of Open CASCADE Technology software library.
//
// This library is free software; you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License version 2.1 as published
// by the Free Software Foundation, with special exception defined in the file
// OCCT_LGPL_EXCEPTION.txt. Consult the file LICENSE_LGPL_21.txt included in OCCT
// distribution for complete text of the license and disclaimer of any warranty.
//
// Alternatively, this file may be used under the terms of Open CASCADE
// commercial license or contractual agreement.

#ifndef _Message_AttributeStream_HeaderFile
#define _Message_AttributeStream_HeaderFile

#include <Message_Attribute.hxx>

#include <Standard_SStream.hxx>

//! Alert object storing stream value
class Message_AttributeStream : public Message_Attribute
{
  DEFINE_STANDARD_RTTIEXT(Message_AttributeStream, Message_Attribute)
public:
  //! Constructor with string argument
  Standard_EXPORT Message_AttributeStream(
    const Standard_SStream&        theStream,
    const TCollection_AsciiString& theName = TCollection_AsciiString());

  //! Returns stream value
  const Standard_SStream& Stream() const { return myStream; }

  //! Sets stream value
  Standard_EXPORT void SetStream(const Standard_SStream& theStream);

  //! Dumps the content of me into the stream
  virtual Standard_EXPORT void DumpJson(Standard_OStream& theOStream,
                                        Standard_Integer  theDepth = -1) const Standard_OVERRIDE;

private:
  Standard_SStream myStream; //!< container of values
};

#endif // _Message_AttributeStream_HeaderFile

```
</details>

<details>
<summary>Searched for regex <code>to_owned|has_to_owned|copy</code> (<code>**/crates/opencascade-binding-generator/src/codegen/bindings.rs</code>), 20 results</summary>

- [bindings.rs:50](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L50)
- [bindings.rs:1068](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1068)
- [bindings.rs:1283](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1283)
- [bindings.rs:1795](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1795)
- [bindings.rs:1796](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1796)
- [bindings.rs:1797](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1797)
- [bindings.rs:1879](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1879)
- [bindings.rs:2155](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2155)
- [bindings.rs:2156](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2156)
- [bindings.rs:2157](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2157)
- [bindings.rs:3239](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3239)
- [bindings.rs:3471](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3471)
- [bindings.rs:3968](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3968)
- [bindings.rs:3969](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3969)
- [bindings.rs:3970](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3970)
- [bindings.rs:4225](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4225)
- [bindings.rs:4342](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4342)
</details>

<details>
<summary>Reading lines 1793-1810 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1793-L1811">bindings.rs</a></summary>

```
    let upcasts = compute_upcast_bindings(class, symbol_table);

    // ── to_owned ────────────────────────────────────────────────────────
    let copyable_modules = ["TopoDS", "gp", "TopLoc", "Bnd", "GProp"];
    let has_to_owned = copyable_modules.contains(&class.module.as_str())
        && !class.has_protected_destructor
        && !effectively_abstract;

    // ── to_handle ───────────────────────────────────────────────────────
    let has_to_handle =
        class.is_handle_type && !class.has_protected_destructor && !effectively_abstract;

    // ── Handle get/get_mut (works for abstract classes too) ─────────────
    let has_handle_get = class.is_handle_type && !class.has_protected_destructor;

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_handle_get {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
```
</details>

> 💭 Based on this thorough research, here's the summary:
> 
> 1. **TDF_LabelNode**: Internal-only,

> 💭 Based on this thorough research, here's the summary:
> 
> 1. **TDF_LabelNode**: Internal-only, never exposed through public APIs. No reason to bind it. The exclude is correct.
> 
> 2. **Poly_CoherentTriPtr**: Has custom allocator hiding standard new/delete. Marginally useful - low-level Poly_CoherentTriangulation subsystem. Could benefit from non-constructible bindings but low priority.
> 
> 3. **TopoDS_AlertAttribute**: Deleted copy constructor (from std::stringstream member). This is a handle type (Standard_Transient subclass), so new/delete work fine. 
> 
> 4. **Message_LazyProgressScope**: Stack-only by design, never in public APIs. Keep excluded.
> 
> The real question is whether to support classes that can't be constructed or destructed. Rather than using the current `has_protected_destructor` flag as a blanket kill switch, I could be more granular: skip constructors and destructors for classes with protected/private destructors where new/delete won't work, but still allow other bindings.
> 
> For handle types specifically, the Handle machinery itself manages deletion through reference counting, so those could still work even with protected destructors. Looking at the three problematic classes—`Message_LazyProgressScope` isn't worth supporting, `Poly_CoherentTriPtr` has hidden allocation operators that would need special handling, and `TDF_LabelNode` has similar issues.
> 
> The real question is whether adding support for non-constructible classes is worth the effort when `Poly_CoherentTriPtr` is the only current beneficiary among the excluded headers. I should report back to the user that `TDF_LabelNode` specifically isn't useful to bind, and let them decide if the broader feature is worth pursuing.

Good findings so far. Let me check how many classes are actually being blanket-skipped due to `has_protected_destructor` across all processed headers — this would tell us if the non-constructible support is worth implementing beyond the 3 excluded headers.

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- 🔄 Check for other excluded non-constructible classes
- ⬜ Implement non-constructible class support
- ⬜ Test and validate changes
- ⬜ Commit changes

<details>
<summary>Searched for text <code>has_protected_destructor</code> (<code>**/crates/opencascade-binding-generator/**</code>), 20 results</summary>

- [resolver.rs:124](../crates/opencascade-binding-generator/src/resolver.rs#L124)
- [resolver.rs:353](../crates/opencascade-binding-generator/src/resolver.rs#L353)
- [resolver.rs:470](../crates/opencascade-binding-generator/src/resolver.rs#L470)
- [resolver.rs:779](../crates/opencascade-binding-generator/src/resolver.rs#L779)
- [resolver.rs:950](../crates/opencascade-binding-generator/src/resolver.rs#L950)
- [resolver.rs:1017](../crates/opencascade-binding-generator/src/resolver.rs#L1017)
- [main.rs:465](../crates/opencascade-binding-generator/src/main.rs#L465)
- [main.rs:649](../crates/opencascade-binding-generator/src/main.rs#L649)
- [main.rs:672](../crates/opencascade-binding-generator/src/main.rs#L672)
- [parser.rs:406](../crates/opencascade-binding-generator/src/parser.rs#L406)
- [parser.rs:413](../crates/opencascade-binding-generator/src/parser.rs#L413)
- [parser.rs:587](../crates/opencascade-binding-generator/src/parser.rs#L587)
- [model.rs:132](../crates/opencascade-binding-generator/src/model.rs#L132)
- [type_mapping.rs:292](../crates/opencascade-binding-generator/src/type_mapping.rs#L292)
- [type_mapping.rs:332](../crates/opencascade-binding-generator/src/type_mapping.rs#L332)
- [bindings.rs:40](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L40)
- [bindings.rs:915](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L915)
- [bindings.rs:1599](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1599)
- [bindings.rs:1798](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1798)
</details>

Let me check how many classes are actually being skipped and see the current protected destructor detection logic:

<details>
<summary>Reading lines 395-600 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L395-L601">parser.rs</a></summary>

```
        }
        return Vec::new();
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    // Extract direct base classes for upcast generation
    let base_classes = extract_base_classes(entity);
    
    // Check for protected/private destructor (indicates non-instantiable abstract base class)
    let has_protected_destructor = check_protected_destructor(entity);

    if verbose {
        println!("  Parsing class: {}", name);
        if !base_classes.is_empty() {
            println!("    Base classes: {:?}", base_classes);
        }
        if has_protected_destructor {
            println!("    Has protected destructor (non-instantiable)");
        }
    }

    let mut constructors = Vec::new();
    let mut methods = Vec::new();
    let mut static_methods = Vec::new();
    let mut fields: Vec<ParsedField> = Vec::new();
    let mut has_non_public_fields = false;
    let mut has_virtual_methods = false;
    let mut all_method_names = std::collections::HashSet::new();
    let mut is_abstract = false;
    let mut pure_virtual_methods = std::collections::HashSet::new();
    let mut has_explicit_constructors = false;
    let mut nested_classes: Vec<ParsedClass> = Vec::new();

    // Track current access level for nested type visibility.
    // Default: `class` => private, `struct` => public.
    let default_access = if entity.get_kind() == EntityKind::StructDecl {
        Accessibility::Public
    } else {
        Accessibility::Private
    };
    let current_access = std::cell::Cell::new(default_access);

    // Check if there's a DEFINE_STANDARD_HANDLE for this class
    // This is typically done outside the class, so we check the name pattern
    // and look for inheritance from Standard_Transient
    let is_handle_type = check_is_handle_type(entity);

    entity.visit_children(|child, _| {
        // Track access specifiers (public:/protected:/private: sections)
        if child.get_kind() == EntityKind::AccessSpecifier {
            if let Some(acc) = child.get_accessibility() {
                current_access.set(acc);
            }
            return EntityVisitResult::Continue;
        }

        match child.get_kind() {
            EntityKind::Constructor => {
                // Any explicit constructor means C++ won't generate an implicit default
                has_explicit_constructors = true;

                // Skip deprecated constructors
                if child.get_availability() == Availability::Deprecated {
                    if verbose {
                        println!("    Skipping deprecated constructor for {}", name);
                    }
                    return EntityVisitResult::Continue;
                }

                if is_public(&child) {
                    if let Some(ctor) = parse_constructor(&child, verbose) {
                        constructors.push(ctor);
                    }
                }
            }
            EntityKind::Method => {
                // Check for virtual methods (affects POD detection)
                if child.is_virtual_method() {
                    has_virtual_methods = true;
                }
                // Check if this is a pure virtual method (makes the class abstract)
                if child.is_pure_virtual_method() {
                    is_abstract = true;
                    if let Some(ref method_name) = child.get_name() {
                        pure_virtual_methods.insert(method_name.clone());
                    }
                }

                // Skip destructors, operators, and conversion functions
                if let Some(ref method_name) = child.get_name() {
                    if method_name.starts_with('~')
                        || method_name.starts_with("operator")
                        || method_name == "DumpJson"
                        || method_name == "InitFromJson"
                    {
                        return EntityVisitResult::Continue;
  
... (truncated, 8116 chars)
```
</details>

<details>
<summary>Reading lines 30-50 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L30-L51">bindings.rs</a></summary>

```
#[derive(Debug, Clone)]
pub struct ClassBindings {
    /// Rust-safe name ("::" flattened to "_" for nested types)
    pub cpp_name: String,
    /// Original C++ qualified name (uses "::" for nested types)
    pub cpp_qualified_name: String,
    pub short_name: String,
    pub module: String,
    pub is_abstract: bool,
    pub is_handle_type: bool,
    pub has_protected_destructor: bool,
    pub doc_comment: Option<String>,
    pub source_header: String,
    pub source_line: Option<u32>,

    pub constructors: Vec<ConstructorBinding>,
    pub direct_methods: Vec<DirectMethodBinding>,
    pub wrapper_methods: Vec<WrapperMethodBinding>,
    pub static_methods: Vec<StaticMethodBinding>,
    pub upcasts: Vec<UpcastBinding>,
    pub has_to_owned: bool,
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01Cyf6Yy8V4aKsHwHWkSFC9A__vscode-1771312782259/content.txt)

Now let me check how many classes are currently skipped due to  in the generated output, and understand the full codegen flow:

<details>
<summary>Reading lines 600-660 of <a href="../crates/opencascade-binding-generator/src/parser.rs#L600-L661">parser.rs</a></summary>

```

    result
}
/// Check if a class has a protected or private destructor
/// Classes with non-public destructors cannot be directly instantiated via the FFI
fn check_protected_destructor(entity: &Entity) -> bool {
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::Destructor {
            // Check if the destructor is not public
            if let Some(accessibility) = child.get_accessibility() {
                return accessibility != clang::Accessibility::Public;
            }
        }
    }
    false
}

/// Parse an enum declaration
fn parse_enum(entity: &Entity, source_header: &str, verbose: bool) -> Option<ParsedEnum> {
    let name = entity.get_name()?;

    // Skip anonymous enums (empty name or compiler-generated "(unnamed enum at ...)")
    if name.is_empty() || name.starts_with("(unnamed") {
        return None;
    }

    // Skip internal enums
    if name.starts_with('_') {
        return None;
    }

    // Skip nested enums (enums defined inside a class/struct)
    // These are not accessible at global scope
    if let Some(parent) = entity.get_semantic_parent() {
        let parent_kind = parent.get_kind();
        if parent_kind == EntityKind::ClassDecl || parent_kind == EntityKind::StructDecl {
            if verbose {
                println!("    Skipping {} (nested enum inside class)", name);
            }
            return None;
        }
    }

    let comment = extract_doxygen_comment(entity);
    let module = extract_module_from_header(source_header);

    if verbose {
        println!("  Parsing enum: {}", name);
    }

    let mut variants = Vec::new();

    entity.visit_children(|child, _| {
        if child.get_kind() == EntityKind::EnumConstantDecl {
            if let Some(variant_name) = child.get_name() {
                let value = child.get_enum_constant_value().map(|(signed, _unsigned)| signed);
                let comment = extract_doxygen_comment(&child);

                if verbose {
                    if let Some(v) = value {
                        println!("    Variant: {} = {}", variant_name, v);
```
</details>

<details>
<summary>Reading lines 900-930 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L900-L931">bindings.rs</a></summary>

```
            } else {
                format!("crate::{}::{}", module_rust, short)
            }
        } else {
            format!("crate::ffi::{}", Type::ffi_safe_class_name(cpp_name))
        }
    }
}

/// Build the class_public_info map from a list of parsed classes.
/// Maps C++ class name → (rust_module_name, short_name).
/// Only includes classes that get `pub use` re-exports (excludes protected-destructor classes).
pub(crate) fn build_class_public_info(all_classes: &[&ParsedClass]) -> HashMap<String, (String, String)> {
    all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| {
            let ffi_name = c.name.replace("::", "_");
            let module_rust = crate::module_graph::module_to_rust_name(&c.module);
            let short = crate::type_mapping::safe_short_name(&crate::type_mapping::short_name_for_module(&ffi_name, &c.module));
            (c.name.clone(), (module_rust, short))
        })
        .collect()
}

/// Convert a Type to Rust type string for re-export files
fn type_to_rust_string(ty: &Type, reexport_ctx: Option<&ReexportTypeContext>) -> String {
    match ty {
        Type::Void => "()".to_string(),
        Type::Bool => "bool".to_string(),
        Type::I32 => "i32".to_string(),
```
</details>

<details>
<summary>Reading lines 1585-1830 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1585-L1831">bindings.rs</a></summary>

```
    reexport_ctx: Option<&ReexportTypeContext>,
    exclude_methods: &HashSet<(String, String)>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    // ── Constructors ────────────────────────────────────────────────────
    let exclude_ctors = exclude_methods.contains(&(class.name.clone(), class.name.clone()))
        || exclude_methods.contains(&(class.name.clone(), "*".to_string()));
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, ncollection_element_types, reexport_ctx);
        if exclude_ctors {
            ctors.clear();
        }
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
                convenience_of: None,
            });
        }
        ctors
    } else {
        Vec::new()
    };

    // ── Direct methods (extern "C" self-pointer wrappers) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| !exclude_methods.contains(&(class.name.clone(), m.name.clone())))
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            DirectMethodBinding {
                rust_name: rust_name.clone(),
                cxx_name: method.name.clone(),
                is_const: method.is_const,
                params,
                return_type,
                doc_comment: method.comment.clone(),
                source_line: method.source_line,
            }
        })
        .collect();

    // ── Wrapper methods (by-value return, const char*) ──────────────────
    let wrapper_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| !exclude_methods.contains(&(class.name.clone(), m.name.clone())))
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let wrapper_fn_names = compute_wrapper_method_names(&wrapper_meth
... (truncated, 10709 chars)
```
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01DdWAJGPMr7T9tG2NV44Pma__vscode-1771312782264/content.txt)

Now let me check how the cpp.rs and rust.rs codegen files skip protected-destructor classes:

<details>
<summary>Reading lines 280-310 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L280-L311">cpp.rs</a></summary>

```
        for (_inner_class, handle_name) in &handle_types {
            writeln!(
                output,
                "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}",
                handle_name, handle_name
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    // Generate wrapper functions for ALL classes from pre-computed ClassBindings
    for bindings in all_bindings {
        if bindings.has_protected_destructor {
            continue;
        }
        output.push_str(&super::bindings::emit_cpp_class(bindings));
    }

    // Generate wrappers for ALL namespace-level free functions
    generate_function_wrappers(&mut output, function_bindings, known_headers);

    // Generate destructors for nested types (e.g., Message_Messenger::StreamBuffer)
    if !nested_types.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "// Nested type destructors").unwrap();
        for nt in nested_types {
            writeln!(
                output,
                "extern \"C\" void {ffi}_destructor({cpp}* self_) {{ delete self_; }}",
                ffi = nt.ffi_name,
```
</details>

<details>
<summary>Reading lines 140-170 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L140-L171">rust.rs</a></summary>

```

    // All enum names (needed for opaque type filtering)
    let all_enum_names = &symbol_table.all_enum_names;

    // Collect collection type names to exclude from class generation
    // Collections are generated separately with specialized wrappers
    let collection_type_names: HashSet<String> = collections.iter()
        .map(|c| c.typedef_name.clone())
        .collect();

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: String = all_bindings
        .iter()
        .filter(|b| !b.has_protected_destructor)
        .filter(|b| !collection_type_names.contains(&b.cpp_name))
        .map(|b| super::bindings::emit_ffi_class(b))
        .collect();

    // Generate namespace-level free functions from pre-computed FunctionBindings
    let function_items = generate_functions_from_bindings(function_bindings);

    // Generate Handle type declarations
    let handle_decls = generate_handle_declarations(all_classes, &symbol_table.handle_able_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(all_classes);
    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
```
</details>

Now let me check what the resolver log says about protected destructors during binding generation:

<details>
<summary>Reading lines 460-490 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L460-L491">resolver.rs</a></summary>

```
    /// Get class by C++ name
    pub fn class_by_name(&self, cpp_name: &str) -> Option<&ResolvedClass> {
        let id = SymbolId::new(format!("class::{}", cpp_name));
        self.classes.get(&id)
    }
    
    /// Get set of all C++ class names that have protected destructors
    pub fn protected_destructor_class_names(&self) -> HashSet<String> {
        self.classes
            .values()
            .filter(|c| c.has_protected_destructor)
            .map(|c| c.cpp_name.clone())
            .collect()
    }
    
    /// Get base classes for a class (recursively collecting all ancestors)
    pub fn get_all_ancestors(&self, class: &ResolvedClass) -> Vec<String> {
        self.get_all_ancestors_by_name(&class.cpp_name)
    }
    
    /// Get all ancestors by C++ class name (recursively collecting all base classes)
    pub fn get_all_ancestors_by_name(&self, cpp_name: &str) -> Vec<String> {
        let mut ancestors = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();
        
        // Start with the direct base classes of the given class
        let mut to_process = std::collections::VecDeque::new();
        if let Some(class) = self.class_by_name(cpp_name) {
            for base in &class.base_classes {
                to_process.push_back(base.clone());
            }
```
</details>

<details>
<summary>Reading lines 940-960 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L940-L961">resolver.rs</a></summary>

```
    class: &ParsedClass,
    all_enum_names: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
) {
    let class_id = SymbolId::new(format!("class::{}", class.name));
    let rust_module = crate::module_graph::module_to_rust_name(&class.module);
    let short_name = crate::type_mapping::short_name_for_module(&class.name, &class.module);
    let rust_ffi_name = safe_short_name(&short_name);
    
    // Determine class binding status
    let class_status = if class.has_protected_destructor {
        BindingStatus::Excluded(ExclusionReason::ProtectedDestructor)
    } else {
        BindingStatus::Included
    };
    
    // Resolve constructors
    let mut constructor_ids = Vec::new();
    for (idx, ctor) in class.constructors.iter().enumerate() {
        let ctor_id = SymbolId::new(format!("ctor::{}::{}", class.name, idx));
        let resolved_ctor = resolve_constructor(
```
</details>

Let me now run the binding generator to see how Let me now run the binding generator to see how many classes are currently being skipped due to :

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -r "protected destructor" crates/opencascade-binding-generator/src/main.rs
```
```
crates/opencascade-binding-generator/src/main.rs:                println!("    [protected destructor]");
```

<details>
<summary>Reading lines 455-480 of <a href="../crates/opencascade-binding-generator/src/main.rs#L455-L481">main.rs</a></summary>

```
        for class in classes {
            let status_str = match &class.status {
                resolver::BindingStatus::Included => "✓ INCLUDED".to_string(),
                resolver::BindingStatus::Excluded(reason) => format!("✗ EXCLUDED: {:?}", reason),
            };
            println!("  CLASS {} ({}) [{}]", class.cpp_name, class.rust_ffi_name, status_str);

            if class.is_abstract {
                println!("    [abstract]");
            }
            if class.has_protected_destructor {
                println!("    [protected destructor]");
            }
            if !class.base_classes.is_empty() {
                println!("    bases: {:?}", class.base_classes);
            }

            // Constructors
            let ctors = table.included_constructors(class);
            let all_ctors: Vec<_> = class.constructors.iter()
                .filter_map(|id| table.constructors.get(id))
                .collect();
            println!("    Constructors: {}/{} included", ctors.len(), all_ctors.len());
            for ctor in all_ctors {
                let ctor_status = match &ctor.status {
                    resolver::BindingStatus::Included => "✓".to_string(),
```
</details>

Let me run the generator with verbose mode to see which classes have protected destructors:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo run -p opencascade-binding-generator -- inspect 2>&1 | grep "protected destructor" | head -50
```
```
(base) whitelily:opencascade-rs sethml$
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo run -p opencascade-binding-generator -- inspect 2>&1 | grep -i "protected.destructor"
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo run -p opencascade-binding-generator -- inspect 2>&1 | grep "ProtectedDestructor"
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo run -p opencascade-binding-generator -- inspect 2>&1 | tee tmp/inspect_output.txt | wc -l
```
```
       5
```

**Terminal:**
```sh
cat tmp/inspect_output.txt
```
<details>
<summary>Output (5 lines)</summary>

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/occt-bindgen inspect`
dyld[39087]: Library not loaded: @rpath/libclang.dylib
  Referenced from: <2BC54647-C27B-3A14-B8F0-29261CD2F707> /Users/sethml/src/opencascade-rs/target/debug/occt-bindgen
  Reason: tried: '/Users/sethml/src/opencascade-rs/target/debug/deps/libclang.dylib' (no such file), '/Users/sethml/src/opencascade-rs/target/debug/libclang.dylib' (no such file), '/Users/sethml/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libclang.dylib' (no such file), '/Users/sethml/.rustup/toolchains/stable-aarch64-apple-darwin/lib/libclang.dylib' (no such file), '/Users/sethml/lib/libclang.dylib' (no such file), '/usr/local/lib/libclang.dylib' (no such file), '/usr/lib/libclang.dylib' (no such file, not in dyld cache)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cat scripts/regenerate-bindings.sh | grep -i "dylib\|DYLD\|rpath\|clang"
```
```
#   - Xcode command line tools (for libclang)
# Set library path for libclang on macOS
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
```

<details>
<summary>Reading lines 1-30 of <a href="../scripts/regenerate-bindings.sh#L1-L31">regenerate-bindings.sh</a></summary>

```
#!/bin/bash
# Regenerate OpenCASCADE bindings
#
# This script regenerates the FFI bindings for opencascade-sys.
# Run from the repository root.
#
# Usage:
#   ./scripts/regenerate-bindings.sh
#
# Requirements:
#   - Xcode command line tools (for libclang)
#   - OCCT headers built in target/OCCT/include

set -euo pipefail

# Repository root (script assumes it's run from repo root)
REPO_ROOT="${REPO_ROOT:-$(pwd)}"
cd "$REPO_ROOT"

# Paths
OCCT_INCLUDE="$REPO_ROOT/target/OCCT/include"
OUTPUT_DIR="$REPO_ROOT/crates/opencascade-sys/generated"
CONFIG_FILE="$REPO_ROOT/crates/opencascade-sys/bindings.toml"

# Check prerequisites
if [[ ! -d "$OCCT_INCLUDE" ]]; then
    echo "Error: OCCT include directory not found at $OCCT_INCLUDE"
    echo "Run 'cargo build -p occt-sys' first to build OCCT"
    exit 1
fi
```
</details>

<details>
<summary>Reading lines 30-60 of <a href="../scripts/regenerate-bindings.sh#L30-L61">regenerate-bindings.sh</a></summary>

```
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "Error: Config file not found at $CONFIG_FILE"
    exit 1
fi

# Build the generator
echo "Building binding generator..."
cargo build --release -p opencascade-binding-generator

echo "Generating bindings from $CONFIG_FILE..."

# Clean generated directory
echo "Cleaning $OUTPUT_DIR..."
rm -f "$OUTPUT_DIR"/*.rs "$OUTPUT_DIR"/*.hxx

# Set library path for libclang on macOS
if [[ "$(uname)" == "Darwin" ]]; then
    XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib"
    export DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}"
fi

# Run the generator
"$REPO_ROOT/target/release/occt-bindgen" \
    --config "$CONFIG_FILE" \
    -I "$OCCT_INCLUDE" \
    -o "$OUTPUT_DIR" \
    "$@"

echo ""
```
</details>

Let me use the proper environment to run the generator and count protected destructor classes:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o /dev/null --verbose 2>&1 | grep -i "protected destructor" | wc -l
```
```
       0
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o /dev/null --verbose 2>&1 | grep -i "Has protected" | wc -l
```
```
       0
```

Hmm, the verbose output might use non-standard paths. Let me check if verbose flag is separate:

Searched for regex `verbose.*flag\|--verbose\|VERBOSE` (`**/crates/opencascade-binding-generator/src/main.rs`), no results

<details>
<summary>Searched for text <code>verbose</code> (<code>**/crates/opencascade-binding-generator/src/main.rs</code>), 15 results</summary>

- [main.rs:42](../crates/opencascade-binding-generator/src/main.rs#L42)
- [main.rs:44](../crates/opencascade-binding-generator/src/main.rs#L44)
- [main.rs:85](../crates/opencascade-binding-generator/src/main.rs#L85)
- [main.rs:107](../crates/opencascade-binding-generator/src/main.rs#L107)
- [main.rs:149](../crates/opencascade-binding-generator/src/main.rs#L149)
- [main.rs:157](../crates/opencascade-binding-generator/src/main.rs#L157)
- [main.rs:160](../crates/opencascade-binding-generator/src/main.rs#L160)
- [main.rs:190](../crates/opencascade-binding-generator/src/main.rs#L190)
- [main.rs:197](../crates/opencascade-binding-generator/src/main.rs#L197)
- [main.rs:199](../crates/opencascade-binding-generator/src/main.rs#L199)
- [main.rs:222](../crates/opencascade-binding-generator/src/main.rs#L222)
- [main.rs:289](../crates/opencascade-binding-generator/src/main.rs#L289)
- [main.rs:342](../crates/opencascade-binding-generator/src/main.rs#L342)
- [main.rs:365](../crates/opencascade-binding-generator/src/main.rs#L365)
- [main.rs:394](../crates/opencascade-binding-generator/src/main.rs#L394)
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o /dev/null -v 2>&1 | grep "protected destructor" | wc -l
```
```
       0
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && XCODE_TOOLCHAIN="$(xcode-select -p)/Toolchains/XcodeDefault.xctoolchain/usr/lib" DYLD_LIBRARY_PATH="$XCODE_TOOLCHAIN:${DYLD_LIBRARY_PATH:-}" cargo run -p opencascade-binding-generator -- --config crates/opencascade-sys/bindings.toml -I target/OCCT/include -o /dev/null -v 2>&1 | grep "non-instantiable" | wc -l
```
```
       0
```

The problem is that the classes with protected destructors are excluded from headers and aren't being parsed at all. Let me take a different approach — add a temporary print to check all parsed classes, or just search the OCCT headers directly to find how many classes in the included modules have protected/private destructors:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -rn "~.*protected\|protected.*~" target/OCCT/include/ 2>/dev/null | grep -v "\.lxx:" | wc -l
```
```
       0
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 -c "
import re, os

# Find classes where the destructor appears after protected/private section
include_dir = 'target/OCCT/include'
results = []

for f in sorted(os.listdir(include_dir)):
    if not f.endswith('.hxx'):
        continue
    path = os.path.join(include_dir, f)
    with open(path) as fh:
        text = fh.read()
    
    # Simple heuristic: find destructor lines and check if preceded by protected/private
    lines = text.split('\n')
    access = 'public'
    classname = None
    for line in lines:
        stripped = line.strip()
        if re.match(r'^class\s+(\w+)', stripped):
            m = re.match(r'^class\s+(\w+)', stripped)
            classname = m.group(1)
            access = 'private'  # default for class
        if stripped == 'public:':
            access = 'public'
        elif stripped == 'protected:':
            access = 'protected'
        elif stripped == 'private:':
            access = 'private'
        if '~' in stripped and access in ('protected', 'private') and classname:
            if re.search(r'~\w+\s*\(', stripped):
                results.append((f, classname, access, stripped[:80]))
                break

for f, cls, acc, line in results:
    print(f'{f}: {cls} ({acc}) {line}')
print(f'\nTotal: {len(results)}')
"
```
```
  File "<string>", line 34
    ppppppppppppppp: {len(results)}')
                                   ^
SyntaxError: unterminated string literal (detected at line 34)
```
**Exit code:** 1

Created [find_protected_dtors.py](../tmp/find_protected_dtors.py)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/find_protected_dtors.py
```
<details>
<summary>Output (35 lines)</summary>

```
AIS_InteractiveContext.hxx: AIS_InteractiveContext (private) Standard_EXPORT virtual ~AIS_InteractiveContext();
BOPAlgo_Algo.hxx: BOPAlgo_Algo (protected) Standard_EXPORT virtual ~BOPAlgo_Algo();
BOPAlgo_BuilderArea.hxx: BOPAlgo_BuilderArea (protected) Standard_EXPORT virtual ~BOPAlgo_BuilderArea();
BOPDS_Interf.hxx: BOPDS_Interf (protected) virtual ~BOPDS_Interf() {}
BRepAlgoAPI_Algo.hxx: BRepAlgoAPI_Algo (protected) Standard_EXPORT virtual ~BRepAlgoAPI_Algo();
BRepMesh_DiscretFactory.hxx: BRepMesh_DiscretFactory (protected) Standard_EXPORT virtual ~BRepMesh_DiscretFactory();
BRepMesh_FaceChecker.hxx: BRepMesh_FaceChecker (private) Standard_EXPORT virtual ~BRepMesh_FaceChecker();
BRepMesh_IncrementalMesh.hxx: BRepMesh_IncrementalMesh (private) Standard_EXPORT virtual ~BRepMesh_IncrementalMesh();
BRepMesh_MeshTool.hxx: NodeClassifier (private) Standard_EXPORT virtual ~BRepMesh_MeshTool();
BVH_LinearBuilder.hxx: BVH_LinearBuilder (protected) BVH_LinearBuilder<T, N>::~BVH_LinearBuilder()
BVH_Traverse.hxx: BVH_BaseTraverse (private) virtual ~BVH_BaseTraverse() {}
BVH_Tree.hxx: BVH_TreeBase (private) virtual ~BVH_TreeBase() {}
CDM_Document.hxx: CDM_Document (protected) Standard_EXPORT ~CDM_Document();
HatchGen_IntersectionPoint.hxx: HatchGen_IntersectionPoint (protected) ~HatchGen_IntersectionPoint() {}
IntCurveSurface_Intersection.hxx: IntCurveSurface_Intersection (protected) ~IntCurveSurface_Intersection() {}
IntRes2d_Intersection.hxx: IntRes2d_Intersection (protected) ~IntRes2d_Intersection() {}
Intf_Interference.hxx: Intf_Interference (protected) ~Intf_Interference() {}
LDOM_DeclareSequence.hxx: ClassName (private) Standard_EXPORT virtual ~ClassName();                                           
LDOM_MemManager.hxx: MemBlock (private) ~MemBlock();
LDOM_OSStream.hxx: LDOM_SBuffer (private) ~LDOM_StringElem();
Message_ProgressScope.hxx: NullString (private) ~Message_ProgressScope()
NCollection_BaseMap.hxx: Iterator (protected) virtual ~NCollection_BaseMap() {}
NCollection_BaseSequence.hxx: Iterator (protected) virtual ~NCollection_BaseSequence() {}
NCollection_CellFilter.hxx: NCollection_CellFilter (protected) aNode->Object.~Target();
NCollection_SparseArray.hxx: Iterator (private) ((TheItemType*)theAddress)->TheItemType::~TheItemType();
NCollection_SparseArrayBase.hxx: Iterator (protected) virtual ~NCollection_SparseArrayBase() { Clear(); }
NCollection_UBTree.hxx: TreeNode (private) virtual ~NCollection_UBTree() { Clear(); }
OSD_FileNode.hxx: OSD_FileNode (protected) ~OSD_FileNode() {}
Standard_ErrorHandler.hxx: Callback (private) inline Standard_ErrorHandler::Callback::~Callback()
TObj_Model.hxx: TObj_Model (protected) Standard_EXPORT ~TObj_Model();
TObj_Persistence.hxx: TObj_Persistence (protected) virtual Standard_EXPORT ~TObj_Persistence();
exptocas.tab.hxx: parser (private) virtual ~parser();
step.tab.hxx: parser (private) virtual ~parser();

Total: 33
```
</details>

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- 🔄 Check for other excluded non-constructible classes
- ⬜ Implement non-constructible class support
- ⬜ Test and validate changes
- ⬜ Commit changes

Now I can see 33 classes with non-public destructors. Several are in modules we bind. Let me filter to just the ones in our included modules:

Created [find_protected_dtors2.py](../tmp/find_protected_dtors2.py)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && python3 tmp/find_protected_dtors2.py
```
<details>
<summary>Output (31 lines)</summary>

```
Classes with non-public destructors in included modules:

  BOPAlgo_Algo.hxx: BOPAlgo_Algo (protected) [INCLUDED (has_protected_destructor applies)]
  BOPAlgo_BuilderArea.hxx: BOPAlgo_BuilderArea (protected) [INCLUDED (has_protected_destructor applies)]
  BOPDS_Interf.hxx: BOPDS_Interf (protected) [INCLUDED (has_protected_destructor applies)]
  BRepAlgoAPI_Algo.hxx: BRepAlgoAPI_Algo (protected) [INCLUDED (has_protected_destructor applies)]
  BRepMesh_DiscretFactory.hxx: BRepMesh_DiscretFactory (protected) [INCLUDED (has_protected_destructor applies)]
  BRepMesh_FaceChecker.hxx: BRepMesh_FaceChecker (private) [INCLUDED (has_protected_destructor applies)]
  BRepMesh_IncrementalMesh.hxx: BRepMesh_IncrementalMesh (private) [INCLUDED (has_protected_destructor applies)]
  BRepMesh_MeshTool.hxx: NodeClassifier (private) [INCLUDED (has_protected_destructor applies)]
  BVH_LinearBuilder.hxx: BVH_LinearBuilder (protected) [INCLUDED (has_protected_destructor applies)]
  BVH_Traverse.hxx: BVH_BaseTraverse (private) [INCLUDED (has_protected_destructor applies)]
  BVH_Tree.hxx: BVH_TreeBase (private) [INCLUDED (has_protected_destructor applies)]
  IntCurveSurface_Intersection.hxx: IntCurveSurface_Intersection (protected) [INCLUDED (has_protected_destructor applies)]
  IntRes2d_Intersection.hxx: IntRes2d_Intersection (protected) [INCLUDED (has_protected_destructor applies)]
  Intf_Interference.hxx: Intf_Interference (protected) [INCLUDED (has_protected_destructor applies)]
  Message_ProgressScope.hxx: NullString (private) [INCLUDED (has_protected_destructor applies)]
  NCollection_BaseMap.hxx: Iterator (protected) [INCLUDED (has_protected_destructor applies)]
  NCollection_BaseSequence.hxx: Iterator (protected) [INCLUDED (has_protected_destructor applies)]
  NCollection_CellFilter.hxx: NCollection_CellFilter (protected) [INCLUDED (has_protected_destructor applies)]
  NCollection_SparseArray.hxx: Iterator (private) [INCLUDED (has_protected_destructor applies)]
  NCollection_SparseArrayBase.hxx: Iterator (protected) [INCLUDED (has_protected_destructor applies)]
  NCollection_UBTree.hxx: TreeNode (private) [INCLUDED (has_protected_destructor applies)]
  OSD_FileNode.hxx: OSD_FileNode (protected) [INCLUDED (has_protected_destructor applies)]
  Standard_ErrorHandler.hxx: Callback (private) [INCLUDED (has_protected_destructor applies)]
  TObj_Model.hxx: TObj_Model (protected) [INCLUDED (has_protected_destructor applies)]
  TObj_Persistence.hxx: TObj_Persistence (protected) [INCLUDED (has_protected_destructor applies)]

Total: 25
Already excluded: 0
Being parsed (blanket-skipped by has_protected_destructor): 25
```
</details>

25 classes being blanket-skipped. Let me understand which of these are important abstract base classes with useful methods.

Analyze 25 protected-destructor classes

Searched for files matching `**/BOPAlgo_Algo.hxx`, 1 match
- [BOPAlgo_Algo.hxx](../crates/occt-sys/OCCT/src/BOPAlgo/BOPAlgo_Algo.hxx)

Searched for files matching `**/BRepAlgoAPI_Algo.hxx`, 1 match
- [BRepAlgoAPI_Algo.hxx](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Algo.hxx)

Searched for files matching `**/BRepMesh_IncrementalMesh.hxx`, 1 match
- [BRepMesh_IncrementalMesh.hxx](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_IncrementalMesh.hxx)

Searched for files matching `**/IntCurveSurface_Intersection.hxx`, 1 match
- [IntCurveSurface_Intersection.hxx](../crates/occt-sys/OCCT/src/IntCurveSurface/IntCurveSurface_Intersection.hxx)

Searched for files matching `**/Message_ProgressScope.hxx`, 1 match
- [Message_ProgressScope.hxx](../crates/occt-sys/OCCT/src/Message/Message_ProgressScope.hxx)

Read [BOPAlgo_Algo.hxx](../crates/occt-sys/OCCT/src/BOPAlgo/BOPAlgo_Algo.hxx)

Read [BOPAlgo_BuilderArea.hxx](../crates/occt-sys/OCCT/src/BOPAlgo/BOPAlgo_BuilderArea.hxx)

Read [BOPDS_Interf.hxx](../crates/occt-sys/OCCT/src/BOPDS/BOPDS_Interf.hxx#1-1), lines 1 to 200

Read [BRepAlgoAPI_Algo.hxx](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Algo.hxx)

Read [BRepMesh_DiscretFactory.hxx](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_DiscretFactory.hxx)

Read [BRepMesh_FaceChecker.hxx](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_FaceChecker.hxx)

Read [BRepMesh_IncrementalMesh.hxx](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_IncrementalMesh.hxx)

Read [BRepMesh_MeshTool.hxx](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_MeshTool.hxx#1-1), lines 1 to 200

Read [IntCurveSurface_Intersection.hxx](../crates/occt-sys/OCCT/src/IntCurveSurface/IntCurveSurface_Intersection.hxx)

Read [IntRes2d_Intersection.hxx](../crates/occt-sys/OCCT/src/IntRes2d/IntRes2d_Intersection.hxx)

Read [Intf_Interference.hxx](../crates/occt-sys/OCCT/src/Intf/Intf_Interference.hxx)

Read [Message_ProgressScope.hxx](../crates/occt-sys/OCCT/src/Message/Message_ProgressScope.hxx#1-1), lines 1 to 300

Read [NCollection_BaseMap.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_BaseMap.hxx#1-1), lines 1 to 150

Read [NCollection_BaseSequence.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_BaseSequence.hxx#1-1), lines 1 to 150

Read [NCollection_CellFilter.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_CellFilter.hxx#1-1), lines 1 to 150

Read [NCollection_SparseArray.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArray.hxx#1-1), lines 1 to 100

Read [NCollection_SparseArrayBase.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_SparseArrayBase.hxx#1-1), lines 1 to 150

Read [NCollection_UBTree.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_UBTree.hxx#1-1), lines 1 to 100

Read [OSD_FileNode.hxx](../crates/occt-sys/OCCT/src/OSD/OSD_FileNode.hxx)

Read [Standard_ErrorHandler.hxx](../crates/occt-sys/OCCT/src/Standard/Standard_ErrorHandler.hxx#1-1), lines 1 to 150

Searched for files matching `**/BVH_LinearBuilder.hxx`, 1 match
- [BVH_LinearBuilder.hxx](../crates/occt-sys/OCCT/src/BVH/BVH_LinearBuilder.hxx)

Searched for files matching `**/BVH_BaseTraverse.hxx`, no matches

Searched for files matching `**/BVH_TreeBase.hxx`, no matches

Searched for files matching `**/TObj_Model.hxx`, 1 match
- [TObj_Model.hxx](../crates/occt-sys/OCCT/src/TObj/TObj_Model.hxx)

Searched for files matching `**/TObj_Persistence.hxx`, 1 match
- [TObj_Persistence.hxx](../crates/occt-sys/OCCT/src/TObj/TObj_Persistence.hxx)

Read [BVH_LinearBuilder.hxx](../crates/occt-sys/OCCT/src/BVH/BVH_LinearBuilder.hxx#1-1), lines 1 to 150

<details>
<summary>Searched for text <code>BVH_BaseTraverse</code> (<code>**/*.hxx</code>), 7 results</summary>

- [BVH_Traverse.hxx:134](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L134)
- [BVH_Traverse.hxx:165](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L165)
- [BVH_Traverse.hxx:168](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L168)
- [BVH_Traverse.hxx:181](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L181)
- [BVH_Traverse.hxx:189](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L189)
- [BVH_Traverse.hxx:255](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L255)
- [BVH_Traverse.hxx:263](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#L263)
</details>

<details>
<summary>Searched for text <code>BVH_TreeBase</code> (<code>**/*.hxx</code>), 10 results</summary>

- [BVH_Tree.hxx:16](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L16)
- [BVH_Tree.hxx:17](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L17)
- [BVH_Tree.hxx:24](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L24)
- [BVH_Tree.hxx:26](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L26)
- [BVH_Tree.hxx:28](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L28)
- [BVH_Tree.hxx:30](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L30)
- [BVH_Tree.hxx:59](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L59)
- [BVH_Tree.hxx:68](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L68)
- [BVH_Tree.hxx:74](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#L74)
</details>

Read [TObj_Model.hxx](../crates/occt-sys/OCCT/src/TObj/TObj_Model.hxx#1-1), lines 1 to 200

Read [TObj_Persistence.hxx](../crates/occt-sys/OCCT/src/TObj/TObj_Persistence.hxx)

Read [BVH_Traverse.hxx](../crates/occt-sys/OCCT/src/BVH/BVH_Traverse.hxx#120-120), lines 120 to 175

Read [BVH_Tree.hxx](../crates/occt-sys/OCCT/src/BVH/BVH_Tree.hxx#1-1), lines 1 to 100

Read [BRepMesh_DiscretRoot.hxx](../crates/occt-sys/OCCT/src/BRepMesh/BRepMesh_DiscretRoot.hxx)

Searched for text `class BOPAlgo_Options`, 3 results
- [BRepAlgoAPI_Algo.hxx:39](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Algo.hxx#L39)
- [boolean_operations.md:551](../crates/occt-sys/OCCT/dox/specification/boolean_operations/boolean_operations.md#L551)
- [BOPAlgo_Options.hxx:36](../crates/occt-sys/OCCT/src/BOPAlgo/BOPAlgo_Options.hxx#L36)

<details>
<summary>Searched for regex <code>class \w+ : public (BRepAlgoAPI_Algo|IntCurveSurface_Intersection|IntRes2d_Intersection|Intf_Interference|OSD_FileNode)</code> (<code>**/*.hxx</code>), 15 results</summary>

- [BRepAlgoAPI_Defeaturing.hxx:100](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Defeaturing.hxx#L100)
- [BRepAlgoAPI_BuilderAlgo.hxx:60](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_BuilderAlgo.hxx#L60)
- [IntPatch_InterferencePolyhedron.hxx:32](../crates/occt-sys/OCCT/src/IntPatch/IntPatch_InterferencePolyhedron.hxx#L32)
- [Intf_InterferencePolygon2d.hxx:32](../crates/occt-sys/OCCT/src/Intf/Intf_InterferencePolygon2d.hxx#L32)
- [IntCurveSurface_HInter.hxx:46](../crates/occt-sys/OCCT/src/IntCurveSurface/IntCurveSurface_HInter.hxx#L46)
- [IntCurveSurface_TheInterferenceOfHInter.hxx:36](../crates/occt-sys/OCCT/src/IntCurveSurface/IntCurveSurface_TheInterferenceOfHInter.hxx#L36)
- [Geom2dInt_TheIntConicCurveOfGInter.hxx:37](../crates/occt-sys/OCCT/src/Geom2dInt/Geom2dInt_TheIntConicCurveOfGInter.hxx#L37)
- [Geom2dInt_TheIntPCurvePCurveOfGInter.hxx:34](../crates/occt-sys/OCCT/src/Geom2dInt/Geom2dInt_TheIntPCurvePCurveOfGInter.hxx#L34)
- [Geom2dInt_IntConicCurveOfGInter.hxx:38](../crates/occt-sys/OCCT/src/Geom2dInt/Geom2dInt_IntConicCurveOfGInter.hxx#L38)
- [Geom2dInt_GInter.hxx:44](../crates/occt-sys/OCCT/src/Geom2dInt/Geom2dInt_GInter.hxx#L44)
- [Geom2dInt_TheIntersectorOfTheIntConicCurveOfGInter.hxx:34](../crates/occt-sys/OCCT/src/Geom2dInt/Geom2dInt_TheIntersectorOfTheIntConicCurveOfGInter.hxx#L34)
- [Bisector_Inter.hxx:32](../crates/occt-sys/OCCT/src/Bisector/Bisector_Inter.hxx#L32)
- [OSD_File.hxx:32](../crates/occt-sys/OCCT/src/OSD/OSD_File.hxx#L32)
- [OSD_Directory.hxx:23](../crates/occt-sys/OCCT/src/OSD/OSD_Directory.hxx#L23)
- [IntCurve_IntConicConic.hxx:39](../crates/occt-sys/OCCT/src/IntCurve/IntCurve_IntConicConic.hxx#L39)
</details>

Searched for regex `BRepMesh_IncrementalMesh|BRepAlgoAPI_Algo|IntCurveSurface_Intersection|IntRes2d_Intersection|BOPAlgo_Algo|Message_ProgressScope` (`**/bindings.toml`), 1 result
- [bindings.toml:226](../crates/opencascade-sys/bindings.toml#L226)

<details>
<summary>Searched for regex <code>BRepAlgoAPI|BRepMesh|BOPAlgo|IntCurveSurface|IntRes2d|Intf_|OSD_File|OSD_Directory|Geom2dInt</code> (<code>**/bindings.toml</code>), 6 results</summary>

- [bindings.toml:95](../crates/opencascade-sys/bindings.toml#L95)
- [bindings.toml:97](../crates/opencascade-sys/bindings.toml#L97)
- [bindings.toml:114](../crates/opencascade-sys/bindings.toml#L114)
- [bindings.toml:125](../crates/opencascade-sys/bindings.toml#L125)
- [bindings.toml:127](../crates/opencascade-sys/bindings.toml#L127)
- [bindings.toml:137](../crates/opencascade-sys/bindings.toml#L137)
</details>

<details>
<summary>Searched for regex <code>class BRepAlgoAPI_BooleanOperation|class BRepAlgoAPI_Cut|class BRepAlgoAPI_Fuse|class BRepAlgoAPI_Common|class BRepAlgoAPI_Section</code> (<code>**/*.hxx</code>), 9 results</summary>

- [BRepAlgoAPI_Fuse.hxx:29](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Fuse.hxx#L29)
- [BRepAlgoAPI_BooleanOperation.hxx:51](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_BooleanOperation.hxx#L51)
- [BRepAlgoAPI_Section.hxx:38](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Section.hxx#L38)
- [BRepAlgoAPI_Cut.hxx:29](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Cut.hxx#L29)
- [BRepAlgoAPI_Common.hxx:29](../crates/occt-sys/OCCT/src/BRepAlgoAPI/BRepAlgoAPI_Common.hxx#L29)
- [BRepFeat_RibSlot.hxx:41](../crates/occt-sys/OCCT/src/BRepFeat/BRepFeat_RibSlot.hxx#L41)
- [BRepFeat_Form.hxx:36](../crates/occt-sys/OCCT/src/BRepFeat/BRepFeat_Form.hxx#L36)
- [DNaming.hxx:40](../crates/occt-sys/OCCT/src/DNaming/DNaming.hxx#L40)
- [DNaming_BooleanOperationDriver.hxx:26](../crates/occt-sys/OCCT/src/DNaming/DNaming_BooleanOperationDriver.hxx#L26)
</details>

Searched for regex `class BRepIntCurveSurface_Inter\b` (`**/*.hxx`), 1 result
- [BRepIntCurveSurface_Inter.hxx:54](../crates/occt-sys/OCCT/src/BRepIntCurveSurface/BRepIntCurveSurface_Inter.hxx#L54)

Read [BOPAlgo_Options.hxx](../crates/occt-sys/OCCT/src/BOPAlgo/BOPAlgo_Options.hxx)

Read [BRepIntCurveSurface_Inter.hxx](../crates/occt-sys/OCCT/src/BRepIntCurveSurface/BRepIntCurveSurface_Inter.hxx#1-1), lines 1 to 100

Read [Message_ProgressScope.hxx](../crates/occt-sys/OCCT/src/Message/Message_ProgressScope.hxx#300-300), lines 300 to 400

Read [NCollection_BaseMap.hxx](../crates/occt-sys/OCCT/src/NCollection/NCollection_BaseMap.hxx#150-150), lines 150 to 253

> 💭 This is very revealing! The key findings:
> 
> 1. **BRepMesh_IncrementalMesh** has a PUBLIC destructor but is being falsely detected as having a protected one. The Python script detected it because of `BRepMesh_MeshTool::NodeClassifier` having a private dtor in the same file, not BRepMesh_IncrementalMesh itself. Let me re-check.
> 
> Wait, actually the Python script was doing a simple heuristic - it might have misreported. The agent's research says the actual class has a public destructor. Let me verify more carefully.
> 
> Actually, the more important point is: the binding generator's `check_protected_destructor()` function uses libclang to check the actual AST, which is more reliable than a regex heuristic. 
> 
> But there's a gap in the logic - the function only examines the class's own destructor, not whether it inherits from a base class with a protected destructor like BRepMesh_DiscretRoot. That inheritance chain isn't being validated by the current check.
> 
> Looking at the actual function, it iterates through direct children to find a destructor and checks its accessibility, returning false if no explicit destructor exists. For BRepMesh_IncrementalMesh with its public virtual destructor, this should return false, yet my Python script flagged it as private. I need to verify whether the destructor is actually in a private section or if there's a discrepancy in how I parsed the header file.
> 
> The key findings are that roughly 8 important classes could benefit from non-constructible bindings, with BRepAlgoAPI_Algo being the most impactful for boolean operations, plus a few intersection-related classes that may be false positives with public destructors. Now I'm thinking through how to adjust the implementation—currently it skips everything when a protected destructor is detected, but I need to refine what should actually be skipped versus what should be allowed.
> 
> For handle types with protected destructors, I can generate Handle<T> wrappers and method bindings since the handle manages the lifetime. For abstract base classes without handle support, I can still generate method wrappers and upcasts even though I can't generate constructors or destructors—the methods would work fine when called through pointers to concrete subclasses, though I'd need to be careful about not generating CppDeletable implementations.
> 
> The key insight is that in the current codegen model, I declare the opaque type, generate method wrappers that take `self_: *` pointers, and skip constructor and destructor generation for abstract bases. I need to verify whether cxx allows opaque types without CppDeletable implementations, since that might be a constraint on what I can generate.
> 
> The inherited method resolution through `compute_inherited_method_bindings()` should handle pulling methods from base classes, but if the base class has no `ClassBindings` generated at all, those methods might not be available to copy down to derived classes. inherit.
> 
> Let me think about this more carefully. The key code flow is:
> 
> 1. `parser.rs` parses all classes including their methods, even for protected-destructor classes
> 2. `bindings.rs` `compute_all_class_bindings()` filters out protected-destructor classes (line 915: `.filter(|c| !c.has_protected_destructor)`)
> 3. `compute_inherited_method_bindings()` tries to find base class methods
> 
> If `compute_all_class_bindings()` skips the entire class, then `compute_inherited_method_bindings()` won't have BRepAlgoAPI_Algo's methods available, and BRepAlgoAPI_Cut won't inherit `Shape()`.
> 
> The issue is that `compute_inherited_method_bindings()` works with the parsed classes which include everything, but if the base class gets filtered out during binding computation, those inherited methods become unavailable. The parser only picks up direct members through `entity.visit_children()`, not inherited ones, so the inherited method resolution is critical. Rather than making the codegen more complex, the simplest approach would be to stop filtering out the protected-destructor classes entirely.
> 
> Instead, I'll skip constructor and destructor generation for those classes, generate everything else like methods and static methods, and handle the CppDeletable trait appropriately based on whether it's a handle type or not.
> 
> Now I'm realizing that Handle<T> can still manage protected-destructor types since Handle's own destructor is public, so I should allow Handle bindings even for those classes. This means I need to make targeted changes across the three files: remove the blanket exclusions in bindings.rs, cpp.rs, and rust.rs, then conditionally skip only the destructor generation for non-handle types while preserving Handle typedef generation for handle types.
> 
> But I'm hitting a deeper issue—for non-handle abstract base types with protected destructors, the Rust bindings would need the opaque type to exist for pointer conversions, yet there's no way to actually construct or destroy instances directly. I need to think through whether methods on such types are even usable in practice.
> 
> Let me look at how the actual code generation works to understand the pattern better.

Excellent findings! Let me check the actual generated code pattern and the inheritance mechanism to understand the full impact:

Check codegen pattern and inherited methods

<details>
<summary>Searched for text <code>compute_inherited_method_bindings</code>, 20 results</summary>

- [2026-02-14_19-00-vscode-claude-opus-4.6.md:172](../agent-logs/2026-02-14_19-00-vscode-claude-opus-4.6.md#L172)
- [2026-02-14_19-00-vscode-claude-opus-4.6.md:399](../agent-logs/2026-02-14_19-00-vscode-claude-opus-4.6.md#L399)
- [2026-02-16_23-50-vscode-claude-opus-4.6.md:441](../agent-logs/2026-02-16_23-50-vscode-claude-opus-4.6.md#L441)
- [2026-02-16_23-50-vscode-claude-opus-4.6.md:477](../agent-logs/2026-02-16_23-50-vscode-claude-opus-4.6.md#L477)
- [2026-02-16_23-50-vscode-claude-opus-4.6.md:575](../agent-logs/2026-02-16_23-50-vscode-claude-opus-4.6.md#L575)
- [2026-02-18-07-30-vscode-claude-opus-4.6.md:1267](../agent-logs/2026-02-18-07-30-vscode-claude-opus-4.6.md#L1267)
- [2026-02-15_07-35-vscode-claude-opus-4.6.md:608](../agent-logs/2026-02-15_07-35-vscode-claude-opus-4.6.md#L608)
- [2026-02-15_07-35-vscode-claude-opus-4.6.md:622](../agent-logs/2026-02-15_07-35-vscode-claude-opus-4.6.md#L622)
- [2026-02-15_07-35-vscode-claude-opus-4.6.md:793](../agent-logs/2026-02-15_07-35-vscode-claude-opus-4.6.md#L793)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1934](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1934)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:1936](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L1936)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:3908](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L3908)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:3917](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L3917)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:3926](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L3926)
- [2026-02-17_21-03-vscode-claude-opus-4.6.md:3963](../agent-logs/2026-02-17_21-03-vscode-claude-opus-4.6.md#L3963)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:731](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L731)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:782](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L782)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:784](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L784)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:875](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L875)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:888](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L888)
</details>

<details>
<summary>Searched for text <code>compute_all_class_bindings</code>, 20 results</summary>

- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1235](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1235)
- [2026-02-16_19-30-vscode-claude-opus-4.6.md:1348](../agent-logs/2026-02-16_19-30-vscode-claude-opus-4.6.md#L1348)
- [claude-conversation-2026-02-13-080b4a40.md:264](../agent-logs/claude-conversation-2026-02-13-080b4a40.md#L264)
- [main.rs:581](../crates/opencascade-binding-generator/src/main.rs#L581)
- [bindings.rs:2746](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2746)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:501](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L501)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:504](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L504)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:506](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L506)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:517](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L517)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:553](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L553)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:590](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L590)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:607](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L607)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:756](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L756)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:769](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L769)
- [2026-02-18-09-30-vscode-claude-opus-4.6.md:1001](../agent-logs/2026-02-18-09-30-vscode-claude-opus-4.6.md#L1001)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:540](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L540)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:580](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L580)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:635](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L635)
- [2026-02-17_07-05-vscode-claude-opus-4.6.md:638](../agent-logs/2026-02-17_07-05-vscode-claude-opus-4.6.md#L638)
</details>

<details>
<summary>Searched for text <code>has_protected_destructor</code> (<code>**/crates/opencascade-binding-generator/**</code>), 20 results</summary>

- [model.rs:132](../crates/opencascade-binding-generator/src/model.rs#L132)
- [resolver.rs:124](../crates/opencascade-binding-generator/src/resolver.rs#L124)
- [resolver.rs:353](../crates/opencascade-binding-generator/src/resolver.rs#L353)
- [resolver.rs:470](../crates/opencascade-binding-generator/src/resolver.rs#L470)
- [resolver.rs:779](../crates/opencascade-binding-generator/src/resolver.rs#L779)
- [resolver.rs:950](../crates/opencascade-binding-generator/src/resolver.rs#L950)
- [resolver.rs:1017](../crates/opencascade-binding-generator/src/resolver.rs#L1017)
- [parser.rs:406](../crates/opencascade-binding-generator/src/parser.rs#L406)
- [parser.rs:413](../crates/opencascade-binding-generator/src/parser.rs#L413)
- [parser.rs:587](../crates/opencascade-binding-generator/src/parser.rs#L587)
- [main.rs:465](../crates/opencascade-binding-generator/src/main.rs#L465)
- [main.rs:649](../crates/opencascade-binding-generator/src/main.rs#L649)
- [main.rs:672](../crates/opencascade-binding-generator/src/main.rs#L672)
- [type_mapping.rs:292](../crates/opencascade-binding-generator/src/type_mapping.rs#L292)
- [type_mapping.rs:332](../crates/opencascade-binding-generator/src/type_mapping.rs#L332)
- [rust.rs:38](../crates/opencascade-binding-generator/src/codegen/rust.rs#L38)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)
- [rust.rs:401](../crates/opencascade-binding-generator/src/codegen/rust.rs#L401)
- [rust.rs:794](../crates/opencascade-binding-generator/src/codegen/rust.rs#L794)
</details>

Searched for text `fn compute_inherited_method_bindings` (`**/codegen/bindings.rs`), 1 result
- [bindings.rs:2438](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2438)

Searched for text `fn compute_all_class_bindings` (`**/codegen/bindings.rs`), 1 result
- [bindings.rs:2746](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2746)

Searched for text `fn emit_cpp_class` (`**/codegen/cpp.rs`), no results

<details>
<summary>Searched for regex <code>CppDeletable|fn generate_|extern &quot;C&quot;</code> (<code>**/codegen/rust.rs</code>), 20 results</summary>

- [rust.rs:1](../crates/opencascade-binding-generator/src/codegen/rust.rs#L1)
- [rust.rs:3](../crates/opencascade-binding-generator/src/codegen/rust.rs#L3)
- [rust.rs:126](../crates/opencascade-binding-generator/src/codegen/rust.rs#L126)
- [rust.rs:130](../crates/opencascade-binding-generator/src/codegen/rust.rs#L130)
- [rust.rs:190](../crates/opencascade-binding-generator/src/codegen/rust.rs#L190)
- [rust.rs:196](../crates/opencascade-binding-generator/src/codegen/rust.rs#L196)
- [rust.rs:198](../crates/opencascade-binding-generator/src/codegen/rust.rs#L198)
- [rust.rs:224](../crates/opencascade-binding-generator/src/codegen/rust.rs#L224)
- [rust.rs:234](../crates/opencascade-binding-generator/src/codegen/rust.rs#L234)
- [rust.rs:282](../crates/opencascade-binding-generator/src/codegen/rust.rs#L282)
- [rust.rs:292](../crates/opencascade-binding-generator/src/codegen/rust.rs#L292)
- [rust.rs:298](../crates/opencascade-binding-generator/src/codegen/rust.rs#L298)
- [rust.rs:326](../crates/opencascade-binding-generator/src/codegen/rust.rs#L326)
- [rust.rs:329](../crates/opencascade-binding-generator/src/codegen/rust.rs#L329)
- [rust.rs:332](../crates/opencascade-binding-generator/src/codegen/rust.rs#L332)
- [rust.rs:358](../crates/opencascade-binding-generator/src/codegen/rust.rs#L358)
- [rust.rs:361](../crates/opencascade-binding-generator/src/codegen/rust.rs#L361)
- [rust.rs:369](../crates/opencascade-binding-generator/src/codegen/rust.rs#L369)
- [rust.rs:395](../crates/opencascade-binding-generator/src/codegen/rust.rs#L395)
- [rust.rs:424](../crates/opencascade-binding-generator/src/codegen/rust.rs#L424)
</details>

<details>
<summary>Reading lines 2438-2600 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2438-L2601">bindings.rs</a></summary>

```
fn compute_inherited_method_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_class_names: &HashSet<String>,
    all_enum_names: &HashSet<String>,
    ncollection_primitive_classes: &HashSet<String>,
    deletable_class_names: Option<&HashSet<String>>,
    reexport_ctx: Option<&ReexportTypeContext>,
) -> Vec<InheritedMethodBinding> {
    if class.has_protected_destructor {
        return Vec::new();
    }

    let existing_method_names: HashSet<String> =
        class.methods.iter().map(|m| m.name.clone()).collect();
    let mut seen_methods: HashSet<String> = HashSet::new();
    // Methods that an intermediate ancestor has re-declared as protected/private.
    // These must not be generated as inherited bindings even if a more-distant
    // ancestor exposes them publicly (e.g. BOPAlgo_PaveFiller narrowing Clear()).
    let mut protected_in_ancestors: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    let ancestors = symbol_table.get_all_ancestors_by_name(&class.name);

    for ancestor_name in &ancestors {
        if let Some(ancestor_class) = symbol_table.class_by_name(ancestor_name) {
            let ancestor_methods = symbol_table.included_methods(ancestor_class);

            // Collect public method names for this ancestor.
            let ancestor_public_names: HashSet<&str> =
                ancestor_methods.iter().map(|m| m.cpp_name.as_str()).collect();
            // Any method declared by this ancestor (in all_method_names) that is
            // NOT publicly exposed has been narrowed (protected/private override).
            // Block it from being inherited from further-up ancestors.
            for method_name in &ancestor_class.all_method_names {
                if !ancestor_public_names.contains(method_name.as_str()) {
                    protected_in_ancestors.insert(method_name.clone());
                }
            }

            for resolved_method in ancestor_methods {
                if existing_method_names.contains(&resolved_method.cpp_name) {
                    continue;
                }
                if class.all_method_names.contains(&resolved_method.cpp_name) {
                    continue;
                }
                if seen_methods.contains(&resolved_method.cpp_name) {
                    continue;
                }
                // Skip methods narrowed to protected/private in an intermediate ancestor.
                // Example: BOPAlgo_PaveFiller overrides BOPAlgo_Options::Clear() as
                // protected; BOPAlgo_CheckerSI must not inherit Clear() from Options.
                if protected_in_ancestors.contains(&resolved_method.cpp_name) {
                    continue;
                }

                seen_methods.insert(resolved_method.cpp_name.clone());

                // Skip methods with raw pointers (but allow nullable pointer params)
                let uses_raw_pointers = resolved_method.params.iter().any(|p| {
                    (p.ty.rust_ffi_type.contains("*const")
                        || p.ty.rust_ffi_type.contains("*mut"))
                        && !p.is_nullable_ptr()
                })
                    || resolved_method
                        .return_type
                        .as_ref()
                        .map(|rt| {
                            rt.rust_ffi_type.contains("*const")
                                || rt.rust_ffi_type.contains("*mut")
                        })
                        .unwrap_or(false);

                if uses_raw_pointers {
                    continue;
                }

                // Skip methods that reference unknown Handle types or unknown classes.
                // But skip this check for params/return types that are enums (they have
                // enum_cpp_name set and are mapped to i32, so they aren't "unknown").
                let uses_unknown_type = resolved_method.params.iter().any(|p| {
            
... (truncated, 7583 chars)
```
</details>

<details>
<summary>Reading lines 2746-2900 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2746-L2901">bindings.rs</a></summary>

```
pub fn compute_all_class_bindings(
    all_classes: &[&ParsedClass],
    symbol_table: &SymbolTable,
    collection_names: &HashSet<String>,
    extra_typedef_names: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the 91 manually-specified known collections (which get generated destructors).
    // NCollection typedef names from extra_typedef_names are NOT included here.
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for class in all_classes {
        for method in &class.methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
        for method in &class.static_methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
    }

    // Full known-type set (for param filtering): adds NCollection template typedefs
    // so methods passing them as params pass the unknown-type filter.
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();

    // Build NCollection class element type maps for misresolution detection
    let (ncollection_element_types, ncollection_primitive_classes) = build_ncollection_element_types(all_classes);

    let class_public_info = build_class_public_info(all_classes);

    all_classes
        .iter()
        .map(|class| {
            let reexport_ctx = ReexportTypeContext {
                class_public_info: &class_public_info,
                current_module_rust: crate::module_graph::module_to_rust_name(&class.module),
            };
            compute_class_bindings(class, &ffi_ctx, symbol_table, &handle_able_classes, &all_classes_by_name, &ncollection_element_types, &ncollection_primitive_classes, Some(&reexport_ctx), exclude_methods)
        })
        .collect()
}

// ── Free function bindings ──────────────────────────────────────────────────

/// Collect nested type names (Parent::Nested) that should be considered dele
... (truncated, 6574 chars)
```
</details>

<details>
<summary>Reading lines 1-200 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L1-L201">rust.rs</a></summary>

```
//! Rust extern "C" FFI code generation
//!
//! Generates the extern "C" FFI module with all OCCT types,
//! plus per-module re-export files with short names and impl blocks.

use crate::model::{ParsedClass, Type};
use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;

/// Generate source attribution for a declaration (header, line number, and C++ identifier)
fn format_source_attribution(header: &str, line: Option<u32>, cpp_name: &str) -> String {
    match line {
        Some(l) => format!("**Source:** `{}`:{} - `{}`", header, l, cpp_name),
        None => format!("**Source:** `{}` - `{}`", header, cpp_name),
    }
}

/// Types collected from class interfaces
pub struct CollectedTypes {
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve") - sorted for deterministic output
    pub classes: BTreeSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>) - sorted for deterministic output
    pub handles: BTreeSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: BTreeSet::new(),
        handles: BTreeSet::new(),
    };

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // This ensures the Handle type is declared even if not used in method signatures
        if class.is_handle_type && !class.has_protected_destructor {
            result.handles.insert(class.name.clone());
        }

        // From constructors
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_types_from_type(&param.ty, &mut result);
            }
        }

        // From methods
        for method in &class.methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }

        // From static methods
        for method in &class.static_methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
            }
        }
    }

    result
}

/// Recursively collect OCCT class and Handle types from a type
fn collect_types_from_type(ty: &Type, collected: &mut CollectedTypes) {
    // Skip unbindable types (arrays, streams, void ptrs, etc.)
    if ty.is_unbindable() {
        return;
    }

    match ty {
        Type::Class(name) => {
            // Skip primitive types that may come from canonical type resolution
            if !is_primitive_type(name) {
                collected.classes.insert(name.clone());
            }
        }
        Type::Handle(name) => {
            // Record the Handle type AND the inner class
            collected.handles.insert(name.clone());
            collected.classes.insert(name.clone());
        }
        Type::ConstRef(inner)
        | Type::MutRef(inner)
        | Type::ConstPtr(inner)
        | Type::MutPtr(inner) => {
            collect_types_from_type(inner, collected);
        }
        _ => {}
    }
}

/// Check if a type name is a primitive (not an OCCT class)
pub fn is_primitive_type(name: &str) -> bool {
    matches!(
        name,
        // Rust primitive names
        "bool" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "char" | "c_char" |
        "c_long" | "c_ulong" |
        // C++ primitive names (may appear from canonical type resolution)
        "double" | "float" | "int" | "unsigned int" | "long" | "unsigned long" |
        "long long" | "unsigned long long" | "short" | "unsigned short" |
        "signed char" | "unsigned char"
    )
}

// ========
... (truncated, 7692 chars)
```
</details>

<details>
<summary>Reading lines 200-500 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L200-L501">rust.rs</a></summary>

```
            writeln!(s, "        {}_destructor(ptr);", nt.ffi_name).unwrap();
            writeln!(s, "    }}").unwrap();
            writeln!(s, "}}").unwrap();
        }
        s
    };

    // Build the output
    let mut out = String::new();

    // File header
    let header_count = all_headers.len();
    writeln!(out, "//! extern \"C\" FFI for OpenCASCADE").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! This file was automatically generated by opencascade-binding-generator").unwrap();
    writeln!(out, "//! from {} OCCT headers.", header_count).unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! Do not edit this file directly.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#![allow(dead_code)]").unwrap();
    writeln!(out, "#![allow(non_snake_case)]").unwrap();
    writeln!(out, "#![allow(clippy::missing_safety_doc)]").unwrap();
    writeln!(out).unwrap();

    // Handle types section (opaque structs outside extern "C")
    if !handle_decls.is_empty() {
        writeln!(out, "// ========================").unwrap();
        writeln!(out, "// Handle types").unwrap();
        writeln!(out, "// ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&handle_decls);
        writeln!(out).unwrap();
    }

    // Class types (opaque structs outside extern "C")
    {
        writeln!(out, "// ========================").unwrap();
        writeln!(out, "// Class types (opaque)").unwrap();
        writeln!(out, "// ========================").unwrap();
        writeln!(out).unwrap();
        for b in all_bindings.iter().filter(|b| !b.is_pod_struct).filter(|b| !collection_type_names.contains(&b.cpp_name)) {
            writeln!(out, "#[repr(C)]").unwrap();
            writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", b.cpp_name).unwrap();
        }
        writeln!(out).unwrap();
    }

    // POD struct types (transparent repr(C) with real fields)
    {
        let pod_structs: Vec<_> = all_bindings.iter().filter(|b| b.is_pod_struct).collect();
        if !pod_structs.is_empty() {
            writeln!(out, "// ========================").unwrap();
            writeln!(out, "// POD struct types").unwrap();
            writeln!(out, "// ========================").unwrap();
            writeln!(out).unwrap();
            for b in &pod_structs {
                writeln!(out, "#[repr(C)]").unwrap();
                writeln!(out, "#[derive(Debug, Clone, Copy)]").unwrap();
                writeln!(out, "pub struct {} {{", b.cpp_name).unwrap();
                for field in &b.pod_fields {
                    if let Some(ref comment) = field.doc_comment {
                        for line in comment.lines() {
                            let trimmed = line.trim();
                            if trimmed.is_empty() {
                                writeln!(out, "    ///").unwrap();
                            } else {
                                writeln!(out, "    /// {}", trimmed).unwrap();
                            }
                        }
                    }
                    if let Some(size) = field.array_size {
                        writeln!(out, "    pub {}: [{}; {}],", field.rust_name, field.rust_type, size).unwrap();
                    } else {
                        writeln!(out, "    pub {}: {},", field.rust_name, field.rust_type).unwrap();
                    }
                }
                writeln!(out, "}}").unwrap();
                writeln!(out).unwrap();
            }
        }
    }

    // Referenced types (opaque structs outside extern "C")
    if !opaque_type_decls.is_empty() {
        writeln!(out, "// ========================").unwrap();
        writeln!(out, "// Referenced types (opaque)").unwrap();
        writeln!(out, "// ========================").unwrap();
        writeln!(out).unwrap();
        out.push_str(&opaque_type_decls);
        writeln!(out).unwrap();
    }

    // Insert collection opaque type declar
... (truncated, 12141 chars)
```
</details>

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-sys/generated/BRepAlgoAPI.rs#L1-L101">BRepAlgoAPI.rs</a></summary>

```
Error reading file: cannot open file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/BRepAlgoAPI.rs. Detail: Unable to read file '/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/BRepAlgoAPI.rs' (Error: Unable to resolve nonexistent file '/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/BRepAlgoAPI.rs')
```
</details>

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-sys/generated/BRepAlgoAPI.hxx#L1-L101">BRepAlgoAPI.hxx</a></summary>

```
Error reading file: cannot open file:///Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/BRepAlgoAPI.hxx. Detail: Unable to read file '/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/BRepAlgoAPI.hxx' (Error: Unable to resolve nonexistent file '/Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/BRepAlgoAPI.hxx')
```
</details>

Searched for files matching `**/generated/BRepAlgoAPI*`, no matches

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01PXyt9ojNjcaQsd2MzCGNcb__vscode-1771312782350/content.txt)

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01BzfxZsX1Zan3DaqBhzChph__vscode-1771312782352/content.txt)

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01KrhBmqd3T6MSPFHjA3UdXA__vscode-1771312782353/content.txt#1-1), lines 1 to 300

<details>
<summary>Reading lines 2600-2745 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2600-L2746">bindings.rs</a></summary>

```
                        // Nullable pointer params: pass through as raw pointers
                        if is_nullable {
                            let (rust_ffi_type, rust_reexport_type, cpp_type) = match &p.ty.original {
                                Type::ConstPtr(inner) => {
                                    let inner_ffi = type_to_ffi_full_name(inner);
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = type_to_cpp(inner);
                                    (
                                        format!("*const {}", inner_ffi),
                                        format!("Option<&{}>", inner_rust),
                                        format!("const {}*", inner_cpp),
                                    )
                                }
                                Type::MutPtr(inner) => {
                                    let inner_ffi = type_to_ffi_full_name(inner);
                                    let inner_rust = type_to_rust_string(inner, reexport_ctx);
                                    let inner_cpp = type_to_cpp(inner);
                                    (
                                        format!("*mut {}", inner_ffi),
                                        format!("Option<&mut {}>", inner_rust),
                                        format!("{}*", inner_cpp),
                                    )
                                }
                                _ => unreachable!("is_nullable_ptr() returned true for non-pointer type"),
                            };
                            return ResolvedParamBinding {
                                name: p.name.clone(),
                                rust_name: p.rust_name.clone(),
                                rust_ffi_type,
                                rust_reexport_type,
                                cpp_type,
                                cpp_arg_expr: p.name.clone(),
                                enum_rust_type: None,
                                mut_ref_enum_cpp_name: None,
                                mut_ref_enum_rust_type: None,
                                is_nullable_ptr: true,
                            };
                        }

                        // Check for &mut enum output params — same as build_param_binding
                        if let Type::MutRef(inner) = &p.ty.original {
                            if let Type::Class(enum_name) = inner.as_ref() {
                                if all_enum_names.contains(enum_name) {
                                    let enum_rust_type = symbol_table.enum_rust_types.get(enum_name).cloned();
                                    let reexport_type = enum_rust_type.as_ref()
                                        .map(|t| format!("&mut {}", t))
                                        .unwrap_or_else(|| "&mut i32".to_string());
                                    return ResolvedParamBinding {
                                        name: p.name.clone(),
                                        rust_name: p.rust_name.clone(),
                                        rust_ffi_type: "&mut i32".to_string(),
                                        rust_reexport_type: reexport_type,
                                        cpp_type: "int32_t&".to_string(),
                                        cpp_arg_expr: format!("{}_enum_", p.name),
                                        enum_rust_type: None,
                                        mut_ref_enum_cpp_name: Some(enum_name.clone()),
                                        mut_ref_enum_rust_type: enum_rust_type,
                                        is_nullable_ptr: false,
                                    };
                                }
                            }
                        }

                        // Convert by-value class/handle params to const ref (same as build_param_bind
... (truncated, 8318 chars)
```
</details>

<details>
<summary>Searched for files matching <code>**/generated/*.rs</code>, 10 matches</summary>

- [geom2d_adaptor.rs](../crates/opencascade-sys/generated/geom2d_adaptor.rs)
- [b_rep_l_prop.rs](../crates/opencascade-sys/generated/b_rep_l_prop.rs)
- [xs_algo.rs](../crates/opencascade-sys/generated/xs_algo.rs)
- [shape_process.rs](../crates/opencascade-sys/generated/shape_process.rs)
- [iges_solid.rs](../crates/opencascade-sys/generated/iges_solid.rs)
- [b_rep_algo.rs](../crates/opencascade-sys/generated/b_rep_algo.rs)
- [i_mesh_data.rs](../crates/opencascade-sys/generated/i_mesh_data.rs)
- [xs_control.rs](../crates/opencascade-sys/generated/xs_control.rs)
- [t_col_geom2d.rs](../crates/opencascade-sys/generated/t_col_geom2d.rs)
- [mat.rs](../crates/opencascade-sys/generated/mat.rs)
</details>

<details>
<summary>Searched for regex <code>fn emit_cpp|destructor|delete|Handle</code> (<code>**/codegen/cpp.rs</code>), 20 results</summary>

- [cpp.rs:16](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L16)
- [cpp.rs:17](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L17)
- [cpp.rs:20](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L20)
- [cpp.rs:21](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L21)
- [cpp.rs:22](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L22)
- [cpp.rs:26](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L26)
- [cpp.rs:28](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L28)
- [cpp.rs:33](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L33)
- [cpp.rs:35](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L35)
- [cpp.rs:41](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L41)
- [cpp.rs:46](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L46)
- [cpp.rs:53](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L53)
</details>

<details>
<summary>Searched for regex <code>has_protected_destructor</code> (<code>**/codegen/rust.rs</code>), 6 results</summary>

- [rust.rs:38](../crates/opencascade-binding-generator/src/codegen/rust.rs#L38)
- [rust.rs:153](../crates/opencascade-binding-generator/src/codegen/rust.rs#L153)
- [rust.rs:401](../crates/opencascade-binding-generator/src/codegen/rust.rs#L401)
- [rust.rs:794](../crates/opencascade-binding-generator/src/codegen/rust.rs#L794)
- [rust.rs:808](../crates/opencascade-binding-generator/src/codegen/rust.rs#L808)
- [rust.rs:822](../crates/opencascade-binding-generator/src/codegen/rust.rs#L822)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01ThRdQnusw3WNGcfGmVFPQB__vscode-1771312782360/content.txt)

Searched for files matching `**/generated/b_rep_algo_api*`, 1 match
- [b_rep_algo_api.rs](../crates/opencascade-sys/generated/b_rep_algo_api.rs)

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L1-L101">cpp.rs</a></summary>

```
//! C++ wrapper code generation
//!
//! Generates extern "C" wrapper functions for all OCCT methods:
//! - Constructors (using placement new or heap allocation)
//! - Return-by-value methods
//! - Static methods
//! - Overloaded methods
//!
//! All wrapper functions use extern "C" linkage for direct FFI access.

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::HashSet;
use std::fmt::Write;

fn collect_handle_types(classes: &[&ParsedClass]) -> Vec<(String, String)> {
    let mut handles = HashSet::new();

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
        }

        for method in &class.methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for method in &class.static_methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }
    }

    let mut result: Vec<_> = handles
        .into_iter()
        .filter(|inner_class| {
            // Skip namespace-scoped types (e.g., IMeshData::ListOfPnt2d) and pointer types
            !inner_class.contains("::") && !inner_class.contains('*') && !inner_class.contains('&')
        })
        .map(|inner_class| {
            // Use full class name to match Rust side (e.g., HandleGeom2dCurve not HandleCurve)
            let handle_name = format!("Handle{}", inner_class.replace("_", ""));
            (inner_class, handle_name)
        })
        .collect();
    result.sort();
    result
}


/// Collect Handle type inner classes from a type
fn collect_type_handles(ty: &Option<Type>, handles: &mut HashSet<String>) {
    if let Some(ty) = ty {
        match ty {
            Type::Handle(name) => {
                handles.insert(name.clone());
            }
            Type::ConstRef(inner) | Type::MutRef(inner) | Type::ConstPtr(inner) | Type::MutPtr(inner) => {
                collect_type_handles(&Some(inner.as_ref().clone()), handles);
            }
            _ => {}
        }
    }
}

/// Collect headers needed for a type
fn collect_type_headers(ty: &Option<Type>, headers: &mut HashSet<String>, known_headers: &HashSet<String>) {
    if let Some(ty) = ty {
        // Skip unbindable types (arrays, streams, void pointers, raw pointers, etc.)
        if ty.is_unbindable() {
            return;
        }

        match ty {
            Type::Class(name) => {
                // Skip primitive types that don't have headers
                // Also skip Standard_Address which is defined in Standard_TypeDef.hxx, not its own file
                if matches!(name.as_str(), 
                    "bool" | "char" | "int" | "unsigned" | "float" | "double" | 
                    "void" | "size_t" | "Standard_Address"
                ) {
                    return;
                }
                // For nested types (Parent::Nested), include the parent class header
                if let Some(parent) = name.split("::").next() {
                    if name.contains("::") {
                        // Nested type — include the parent's header
                        if parent.contains('_') || parent.starts_with("Standard") {
```
</details>

<details>
<summary>Reading lines 780-850 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L780-L851">rust.rs</a></summary>

```
    if !collections.is_empty() {
        output.push('\n');
    }

    // Generate impl blocks for collection types
    for coll in collections {
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
                base_handle_reexports.insert(hd.derived_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Handle type re-exports (targets of handle upcasts/downcasts)\n");
        for handle_name in &base_handle_reexports {
            output.push_str(&format!("pub use crate::ffi::{};\n", handle_name));
        }
        output.push_str("\n");
    }

    for (header, header_bindings) in bindings_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");
```
</details>

<details>
<summary>Reading lines 1-100 of <a href="../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1-L101">b_rep_algo_api.rs</a></summary>

```
//! BRepAlgoAPI module re-exports
//!
//! This file was automatically generated by opencascade-binding-generator.
//! Do not edit this file directly.

#![allow(dead_code)]
#![allow(non_snake_case)]

// ========================
// From BRepAlgoAPI_BooleanOperation.hxx
// ========================

/// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:51 - `BRepAlgoAPI_BooleanOperation`
/// The root API class for performing Boolean Operations on arbitrary shapes.
///
/// The arguments of the operation are divided in two groups - *Objects* and *Tools*.
/// Each group can contain any number of shapes, but each shape should be valid
/// in terms of *BRepCheck_Analyzer* and *BOPAlgo_ArgumentAnalyzer*.
/// The algorithm builds the splits of the given arguments using the intersection
/// results and combines the result of Boolean Operation of given type:
/// - *FUSE* - union of two groups of objects;
/// - *COMMON* - intersection of two groups of objects;
/// - *CUT* - subtraction of one group from the other;
/// - *SECTION* - section edges and vertices of all arguments;
///
/// The rules for the arguments and type of the operation are the following:
/// - For Boolean operation *FUSE* all arguments should have equal dimensions;
/// - For Boolean operation *CUT* the minimal dimension of *Tools* should not be
/// less than the maximal dimension of *Objects*;
/// - For Boolean operation *COMMON* the arguments can have any dimension.
/// - For Boolean operation *SECTION* the arguments can be of any type.
///
/// Additionally to the errors of the base class the algorithm returns
/// the following Errors:<br>
/// - *BOPAlgo_AlertBOPNotSet* - in case the type of Boolean Operation is not set.<br>
pub use crate::ffi::BRepAlgoAPI_BooleanOperation as BooleanOperation;

unsafe impl crate::CppDeletable for BooleanOperation {
    unsafe fn cpp_delete(ptr: *mut Self) {
        crate::ffi::BRepAlgoAPI_BooleanOperation_destructor(ptr);
    }
}

impl BooleanOperation {
    /// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:58 - `BRepAlgoAPI_BooleanOperation::BRepAlgoAPI_BooleanOperation()`
    /// @name Constructors
    /// Empty constructor
    pub fn new() -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::BRepAlgoAPI_BooleanOperation_ctor()) }
    }

    /// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:61 - `BRepAlgoAPI_BooleanOperation::BRepAlgoAPI_BooleanOperation()`
    /// Constructor with precomputed intersections of arguments.
    pub fn new_pavefiller(thePF: &crate::bop_algo::PaveFiller) -> crate::OwnedPtr<Self> {
        unsafe {
            crate::OwnedPtr::from_raw(crate::ffi::BRepAlgoAPI_BooleanOperation_ctor_pavefiller(
                thePF,
            ))
        }
    }

    /// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:66 - `BRepAlgoAPI_BooleanOperation::Shape1()`
    /// @name Setting/getting arguments
    /// Returns the first argument involved in this Boolean operation.
    /// Obsolete
    pub fn shape1(&self) -> &crate::topo_ds::Shape {
        unsafe { &*(crate::ffi::BRepAlgoAPI_BooleanOperation_shape1(self as *const Self)) }
    }

    /// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:70 - `BRepAlgoAPI_BooleanOperation::Shape2()`
    /// Returns the second argument involved in this Boolean operation.
    /// Obsolete
    pub fn shape2(&self) -> &crate::topo_ds::Shape {
        unsafe { &*(crate::ffi::BRepAlgoAPI_BooleanOperation_shape2(self as *const Self)) }
    }

    /// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:73 - `BRepAlgoAPI_BooleanOperation::SetTools()`
    /// Sets the Tool arguments
    pub fn set_tools(&mut self, theLS: &crate::ffi::TopTools_ListOfShape) {
        unsafe { crate::ffi::BRepAlgoAPI_BooleanOperation_set_tools(self as *mut Self, theLS) }
    }

    /// **Source:** `BRepAlgoAPI_BooleanOperation.hxx`:76 - `BRepAlgoAPI_BooleanOperation::Tools()`
    /// Returns the Tools arguments
    pub fn tools(&self) -> &crate::ffi::TopTools_ListOfShape {
        unsafe { &*(cra
... (truncated, 4703 chars)
```
</details>

<details>
<summary>Searched for text <code>Shape</code> (<code>**/generated/b_rep_algo_api.rs</code>), 20 results</summary>

- [b_rep_algo_api.rs:14](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L14)
- [b_rep_algo_api.rs:17](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L17)
- [b_rep_algo_api.rs:62](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L62)
- [b_rep_algo_api.rs:66](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L66)
- [b_rep_algo_api.rs:67](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L67)
- [b_rep_algo_api.rs:70](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L70)
- [b_rep_algo_api.rs:73](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L73)
- [b_rep_algo_api.rs:74](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L74)
- [b_rep_algo_api.rs:79](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L79)
- [b_rep_algo_api.rs:85](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L85)
- [b_rep_algo_api.rs:134](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L134)
- [b_rep_algo_api.rs:135](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L135)
- [b_rep_algo_api.rs:137](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L137)
- [b_rep_algo_api.rs:143](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L143)
- [b_rep_algo_api.rs:144](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L144)
- [b_rep_algo_api.rs:146](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L146)
</details>

Searched for regex `fn emit_cpp_|generate_cpp|fn generate` (`**/codegen/cpp.rs`), 3 results
- [cpp.rs:136](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L136)
- [cpp.rs:237](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L237)
- [cpp.rs:319](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L319)

Searched for text `fn emit_ffi_class` (`**/codegen/bindings.rs`), 1 result
- [bindings.rs:4828](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4828)

<details>
<summary>Reading lines 4828-5000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4828-L5001">bindings.rs</a></summary>

```
pub fn emit_ffi_class(bindings: &ClassBindings) -> String {
    // POD structs are defined as #[repr(C)] with real fields — they only
    // need a sizeof helper for layout verification.
    if bindings.is_pod_struct {
        let cn = &bindings.cpp_name;
        let mut out = String::new();
        writeln!(out, "    // ======================== {} (POD) ========================", cn).unwrap();
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
        writeln!(out, "    pub fn {}_sizeof() -> usize;", cn).unwrap();
        return out;
    }

    let cn = &bindings.cpp_name;
    let mut out = String::new();

    // Section header
    writeln!(out, "    // ======================== {} ========================", cn).unwrap();

    // ── Destructor ──────────────────────────────────────────────────────
    if !bindings.has_protected_destructor {
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
    }

    // ── Constructors (skip convenience wrappers — they are Rust-only) ──
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let source = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        emit_ffi_doc_4(&mut out, &source, &ctor.doc_comment);

        let params_str = format_params(&ctor.params);
        writeln!(out, "    pub fn {}({}) -> *mut {};", ctor.ffi_fn_name, params_str, cn).unwrap();
    }

    // ── Direct methods — with extern "C", these become wrapper functions too ──
    for dm in &bindings.direct_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            dm.source_line,
            &format!("{}::{}()", cn, dm.cxx_name),
        );
        emit_ffi_doc_4(&mut out, &source, &dm.doc_comment);

        let self_param = if dm.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str = format_params(&dm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = format_return_type(&dm.return_type);
        writeln!(out, "    pub fn {}_{}({}){};", cn, dm.rust_name, all_params, ret).unwrap();
    }

    // ── Wrapper methods (free functions with self_ parameter) ────────────
    for wm in &bindings.wrapper_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            wm.source_line,
            &format!("{}::{}()", cn, wm.cpp_method_name),
        );
        emit_ffi_doc_4(&mut out, &source, &wm.doc_comment);

        let self_param = if wm.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str = format_params(&wm.params);
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = format_return_type(&wm.return_type);
        writeln!(out, "    pub fn {}({}){};", wm.ffi_fn_name, all_params, ret).unwrap();
    }

    // ── Static methods ──────────────────────────────────────────────────
    for sm in &bindings.static_methods {
        let source = format_source_attribution(
            &bindings.source_header,
            sm.source_line,
            &format!("{}::{}()", cn, sm.cpp_method_name),
        );
        emit_ffi_doc_4(&mut out, &source, &sm.doc_comment);

        let params_str = format_params(&sm.params);
        let ret = if let Some(ref rt) = sm.return_type {
            format!(" -> {}", rt.rust_ffi_type)
        } else {
            String::new()
        };
        writeln!(out, "    pub fn {}({}){};", sm.ffi_fn_name, params_str, ret).unwrap();
    }

    // ── Upcasts ──────────────────────────
... (truncated, 7670 chars)
```
</details>

<details>
<summary>Reading lines 136-380 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L136-L381">cpp.rs</a></summary>

```
fn generate_function_wrappers(
    output: &mut String,
    function_bindings: &[super::bindings::FunctionBinding],
    known_headers: &HashSet<String>,
) {
    if function_bindings.is_empty() {
        return;
    }

    // Group functions by namespace
    let mut by_namespace: std::collections::HashMap<&str, Vec<&super::bindings::FunctionBinding>> =
        std::collections::HashMap::new();
    for func in function_bindings {
        by_namespace
            .entry(&func.namespace)
            .or_default()
            .push(func);
    }

    let mut namespaces: Vec<&&str> = by_namespace.keys().collect();
    namespaces.sort();

    for namespace in namespaces {
        let namespace_functions = &by_namespace[namespace];

        writeln!(output, "// ========================").unwrap();
        writeln!(output, "// {} namespace functions", namespace).unwrap();
        writeln!(output, "// ========================").unwrap();

        // Collect unique headers for this namespace
        let mut extra_headers: HashSet<String> = HashSet::new();
        let ns_header = format!("{}.hxx", namespace);
        if known_headers.is_empty() || known_headers.contains(&ns_header) {
            extra_headers.insert(ns_header);
        }
        for func in namespace_functions {
            for h in &func.cpp_headers {
                extra_headers.insert(h.clone());
            }
        }

        let mut sorted_headers: Vec<_> = extra_headers.into_iter().collect();
        sorted_headers.sort();
        for header in &sorted_headers {
            writeln!(output, "#include <{}>", header).unwrap();
        }

        for func in namespace_functions {
            let wrapper_name = &func.cpp_wrapper_name;

            // Build param declarations from pre-computed cpp_type
            let params_cpp: Vec<String> = func.params.iter()
                .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
                .collect();
            let params_str = params_cpp.join(", ");

            // Build argument expressions from pre-computed cpp_arg_expr
            let args: Vec<String> = func.params.iter()
                .map(|p| p.cpp_arg_expr.clone())
                .collect();
            let args_str = args.join(", ");

            let call = format!("{}::{}({})", namespace, func.short_name, args_str);

            // Determine return pattern from pre-computed return type binding
            if let Some(ref rt) = func.return_type {
                if rt.enum_cpp_name.is_some() {
                    writeln!(
                        output,
                        "extern \"C\" {} {}({}) {{ return static_cast<int32_t>({}); }}",
                        rt.cpp_type, wrapper_name, params_str, call
                    ).unwrap();
                } else if rt.needs_unique_ptr {
                    // Return type is the base C++ type; wrapper returns pointer
                    // cpp_type for unique_ptr returns is the base type (e.g. "gp_Pnt")
                    // but the FFI returns a pointer to it
                    let base_type = &rt.cpp_type;
                    writeln!(
                        output,
                        "extern \"C\" {0}* {1}({2}) {{ return new {0}({3}); }}",
                        base_type, wrapper_name, params_str, call
                    ).unwrap();
                } else {
                    writeln!(
                        output,
                        "extern \"C\" {} {}({}) {{ return {}; }}",
                        rt.cpp_type, wrapper_name, params_str, call
                    ).unwrap();
                }
            } else {
                writeln!(
                    output,
                    "extern \"C\" void {}({}) {{ {}; }}",
                    wrapper_name, params_str, call
                ).unwrap();
            }
        }
        writeln!(output).unwrap();
    }
}

pub fn generate_wrappers(
    all_classes: &[&ParsedClass],
    collections: &[super::collections::CollectionInfo],

... (truncated, 8653 chars)
```
</details>

<details>
<summary>Searched for text <code>BRepAlgoAPI_Cut</code> (<code>**/generated/b_rep_algo_api.rs</code>), 20 results</summary>

- [b_rep_algo_api.rs:1240](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1240)
- [b_rep_algo_api.rs:1243](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1243)
- [b_rep_algo_api.rs:1246](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1246)
- [b_rep_algo_api.rs:1250](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1250)
- [b_rep_algo_api.rs:1255](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1255)
- [b_rep_algo_api.rs:1258](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1258)
- [b_rep_algo_api.rs:1261](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1261)
- [b_rep_algo_api.rs:1265](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1265)
- [b_rep_algo_api.rs:1268](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1268)
- [b_rep_algo_api.rs:1280](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1280)
- [b_rep_algo_api.rs:1286](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1286)
</details>

<details>
<summary>Searched for text <code>inherited</code> (<code>**/generated/b_rep_algo_api.rs</code>), 20 results</summary>

- [b_rep_algo_api.rs:172](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L172)
- [b_rep_algo_api.rs:175](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L175)
- [b_rep_algo_api.rs:182](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L182)
- [b_rep_algo_api.rs:185](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L185)
- [b_rep_algo_api.rs:189](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L189)
- [b_rep_algo_api.rs:192](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L192)
- [b_rep_algo_api.rs:199](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L199)
- [b_rep_algo_api.rs:202](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L202)
- [b_rep_algo_api.rs:206](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L206)
- [b_rep_algo_api.rs:209](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L209)
- [b_rep_algo_api.rs:216](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L216)
- [b_rep_algo_api.rs:220](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L220)
- [b_rep_algo_api.rs:226](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L226)
- [b_rep_algo_api.rs:229](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L229)
- [b_rep_algo_api.rs:236](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L236)
- [b_rep_algo_api.rs:239](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L239)
- [b_rep_algo_api.rs:243](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L243)
- [b_rep_algo_api.rs:251](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L251)
- [b_rep_algo_api.rs:260](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L260)
- [b_rep_algo_api.rs:263](../crates/opencascade-sys/generated/b_rep_algo_api.rs#L263)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01WnKDNx8yoR1zFeqZiLTk8T__vscode-1771312782372/content.txt)

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01XvysDcuFsKsyRTMbWoQedq__vscode-1771312782373/content.txt)

<details>
<summary>Reading lines 1240-1450 of <a href="../crates/opencascade-sys/generated/b_rep_algo_api.rs#L1240-L1451">b_rep_algo_api.rs</a></summary>

```
// From BRepAlgoAPI_Cut.hxx
// ========================

/// **Source:** `BRepAlgoAPI_Cut.hxx`:29 - `BRepAlgoAPI_Cut`
/// The class Cut provides Boolean cut operation
/// between arguments and tools (Boolean Subtraction).
pub use crate::ffi::BRepAlgoAPI_Cut as Cut;

unsafe impl crate::CppDeletable for Cut {
    unsafe fn cpp_delete(ptr: *mut Self) {
        crate::ffi::BRepAlgoAPI_Cut_destructor(ptr);
    }
}

impl Cut {
    /// **Source:** `BRepAlgoAPI_Cut.hxx`:35 - `BRepAlgoAPI_Cut::BRepAlgoAPI_Cut()`
    /// Empty constructor
    pub fn new() -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::BRepAlgoAPI_Cut_ctor()) }
    }

    /// **Source:** `BRepAlgoAPI_Cut.hxx`:40 - `BRepAlgoAPI_Cut::BRepAlgoAPI_Cut()`
    /// Empty constructor
    /// <PF> - PaveFiller object that is carried out
    pub fn new_pavefiller(PF: &crate::bop_algo::PaveFiller) -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::BRepAlgoAPI_Cut_ctor_pavefiller(PF)) }
    }

    /// **Source:** `BRepAlgoAPI_Cut.hxx`:47 - `BRepAlgoAPI_Cut::BRepAlgoAPI_Cut()`
    /// Constructor with two shapes
    /// <S1>  -argument
    /// <S2>  -tool
    /// <anOperation> - the type of the operation
    /// Obsolete
    pub fn new_shape2_progressrange(
        S1: &crate::topo_ds::Shape,
        S2: &crate::topo_ds::Shape,
        theRange: &crate::message::ProgressRange,
    ) -> crate::OwnedPtr<Self> {
        unsafe {
            crate::OwnedPtr::from_raw(crate::ffi::BRepAlgoAPI_Cut_ctor_shape2_progressrange(
                S1, S2, theRange,
            ))
        }
    }

    /// **Source:** `BRepAlgoAPI_Cut.hxx`:57 - `BRepAlgoAPI_Cut::BRepAlgoAPI_Cut()`
    /// Constructor with two shapes
    /// <S1>  -argument
    /// <S2>  -tool
    /// <anOperation> - the type of the operation
    /// <PF> - PaveFiller object that is carried out
    /// Obsolete
    pub fn new_shape2_pavefiller_bool_progressrange(
        S1: &crate::topo_ds::Shape,
        S2: &crate::topo_ds::Shape,
        aDSF: &crate::bop_algo::PaveFiller,
        bFWD: bool,
        theRange: &crate::message::ProgressRange,
    ) -> crate::OwnedPtr<Self> {
        unsafe {
            crate::OwnedPtr::from_raw(
                crate::ffi::BRepAlgoAPI_Cut_ctor_shape2_pavefiller_bool_progressrange(
                    S1, S2, aDSF, bFWD, theRange,
                ),
            )
        }
    }

    /// Upcast to BRepAlgoAPI_BooleanOperation
    pub fn as_boolean_operation(&self) -> &BooleanOperation {
        unsafe {
            &*(crate::ffi::BRepAlgoAPI_Cut_as_BRepAlgoAPI_BooleanOperation(self as *const Self))
        }
    }

    /// Upcast to BRepAlgoAPI_BooleanOperation (mutable)
    pub fn as_boolean_operation_mut(&mut self) -> &mut BooleanOperation {
        unsafe {
            &mut *(crate::ffi::BRepAlgoAPI_Cut_as_BRepAlgoAPI_BooleanOperation_mut(
                self as *mut Self,
            ))
        }
    }

    /// Upcast to BRepAlgoAPI_BuilderAlgo
    pub fn as_builder_algo(&self) -> &BuilderAlgo {
        unsafe { &*(crate::ffi::BRepAlgoAPI_Cut_as_BRepAlgoAPI_BuilderAlgo(self as *const Self)) }
    }

    /// Upcast to BRepAlgoAPI_BuilderAlgo (mutable)
    pub fn as_builder_algo_mut(&mut self) -> &mut BuilderAlgo {
        unsafe {
            &mut *(crate::ffi::BRepAlgoAPI_Cut_as_BRepAlgoAPI_BuilderAlgo_mut(self as *mut Self))
        }
    }

    /// Upcast to BRepBuilderAPI_MakeShape
    pub fn as_b_rep_builder_api_make_shape(&self) -> &crate::b_rep_builder_api::MakeShape {
        unsafe { &*(crate::ffi::BRepAlgoAPI_Cut_as_BRepBuilderAPI_MakeShape(self as *const Self)) }
    }

    /// Upcast to BRepBuilderAPI_MakeShape (mutable)
    pub fn as_b_rep_builder_api_make_shape_mut(
        &mut self,
    ) -> &mut crate::b_rep_builder_api::MakeShape {
        unsafe {
            &mut *(crate::ffi::BRepAlgoAPI_Cut_as_BRepBuilderAPI_MakeShape_mut(self as *mut Self))
        }
    }

    /// Upcast to BRepBuilderAPI_Command
    pub f
... (truncated, 8735 chars)
```
</details>

Searched for text `fn emit_cpp_class` (`**/codegen/bindings.rs`), 1 result
- [bindings.rs:3137](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3137)

Searched for text `fn handle_able_classes` (`**/resolver.rs`), no results

Searched for text `has_protected_destructor` (`**/main.rs`), 3 results
- [main.rs:465](../crates/opencascade-binding-generator/src/main.rs#L465)
- [main.rs:649](../crates/opencascade-binding-generator/src/main.rs#L649)
- [main.rs:672](../crates/opencascade-binding-generator/src/main.rs#L672)

<details>
<summary>Reading lines 3137-3350 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3137-L3351">bindings.rs</a></summary>

```
pub fn emit_cpp_class(bindings: &ClassBindings) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    let ffi_cn = &bindings.cpp_name;  // Rust-safe flattened name (for FFI function names)
    let cn = &bindings.cpp_qualified_name;  // C++ qualified name (for C++ type expressions)

    // POD structs don't need C++ wrappers, but we generate a sizeof helper
    // so Rust tests can verify layout compatibility at runtime,
    // and a destructor so CppDeletable can be implemented (needed when returned by pointer).
    if bindings.is_pod_struct {
        writeln!(output, "// sizeof helper for POD struct {}", cn).unwrap();
        writeln!(output, "extern \"C\" size_t {}_sizeof() {{ return sizeof({}); }}", ffi_cn, cn).unwrap();
        writeln!(output, "extern \"C\" void {}_destructor({}* self_) {{ delete self_; }}", ffi_cn, cn).unwrap();
        writeln!(output).unwrap();
        return output;
    }

    writeln!(output, "// ========================").unwrap();
    writeln!(output, "// {} wrappers", cn).unwrap();
    writeln!(output, "// ========================").unwrap();
    writeln!(output).unwrap();

    // 1. Constructor wrappers (skip convenience — they are Rust-only)
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let params_cpp: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let params_str = params_cpp.join(", ");
        let args_str = ctor.cpp_arg_exprs.join(", ");

        writeln!(
            output,
            "extern \"C\" {cn}* {fn_name}({params_str}) {{",
            fn_name = ctor.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {cn}({args_str});"
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 2. ByValueReturn wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::ByValueReturn)
    {
        let self_param = if wm.is_const {
            format!("const {cn}* self_")
        } else {
            format!("{cn}* self_")
        };

        let other_params: Vec<String> = wm
            .params
            .iter()
            .map(|p| format!("{} {}", p.cpp_type, p.cpp_name))
            .collect();
        let all_params = std::iter::once(self_param)
            .chain(other_params)
            .collect::<Vec<_>>()
            .join(", ");
        let args_str = wm
            .params
            .iter()
            .map(|p| p.cpp_arg_expr.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let ret_cpp = &wm.return_type.as_ref().unwrap().cpp_type;

        writeln!(
            output,
            "extern \"C\" {ret_cpp}* {fn_name}({all_params}) {{",
            fn_name = wm.ffi_fn_name
        )
        .unwrap();
        writeln!(
            output,
            "    return new {ret_cpp}(self_->{method}({args_str}));",
            method = wm.cpp_method_name
        )
        .unwrap();
        writeln!(output, "}}").unwrap();
    }

    // 3. Static method wrappers
    // Note: In the old code, static methods were emitted between by-value and cstring wrappers
    // when you look at the call order in generate_class_wrappers. Actually, the order is:
    // by-value → cstring-param → cstring-return → static. Let me re-check...
    // The actual call order in generate_class_wrappers is:
    //   1. constructor
    //   2. return_by_value
    //   3. c_string_param
    //   4. c_string_return
    //   5. static_method
    //   6. upcast
    //   7. to_owned
    //   8. to_handle
    //   9. handle_upcast
    //   9b. handle_downcast
    //   10. inherited_method

    // 3. CStringParam wrapper methods
    for wm in bindings
        .wrapper_methods
        .iter()
        .filter(|m| m.wrapper_kind == WrapperKind::CStringParam)
    {
        let se
... (truncated, 7134 chars)
```
</details>

<details>
<summary>Reading lines 450-700 of <a href="../crates/opencascade-binding-generator/src/main.rs#L450-L701">main.rs</a></summary>

```
    for module in modules {
        println!("=== Module: {} ===\n", module);

        // Dump classes in this module
        let classes = table.classes_for_module(module);
        for class in classes {
            let status_str = match &class.status {
                resolver::BindingStatus::Included => "✓ INCLUDED".to_string(),
                resolver::BindingStatus::Excluded(reason) => format!("✗ EXCLUDED: {:?}", reason),
            };
            println!("  CLASS {} ({}) [{}]", class.cpp_name, class.rust_ffi_name, status_str);

            if class.is_abstract {
                println!("    [abstract]");
            }
            if class.has_protected_destructor {
                println!("    [protected destructor]");
            }
            if !class.base_classes.is_empty() {
                println!("    bases: {:?}", class.base_classes);
            }

            // Constructors
            let ctors = table.included_constructors(class);
            let all_ctors: Vec<_> = class.constructors.iter()
                .filter_map(|id| table.constructors.get(id))
                .collect();
            println!("    Constructors: {}/{} included", ctors.len(), all_ctors.len());
            for ctor in all_ctors {
                let ctor_status = match &ctor.status {
                    resolver::BindingStatus::Included => "✓".to_string(),
                    resolver::BindingStatus::Excluded(reason) => format!("✗ {:?}", reason),
                };
                let params: Vec<_> = ctor.params.iter().map(|p| format!("{}: {}", p.rust_name, p.ty.rust_ffi_type)).collect();
                println!("      {} {}({}) [{}]", ctor_status, ctor.rust_name, params.join(", "),
                    if ctor.status.is_included() { "included" } else { "excluded" });
            }

            // Methods
            let methods = table.included_methods(class);
            let all_methods: Vec<_> = class.methods.iter()
                .filter_map(|id| table.methods.get(id))
                .collect();
            println!("    Methods: {}/{} included", methods.len(), all_methods.len());

            // Show excluded methods with reasons
            for method in all_methods.iter().filter(|m| m.status.is_excluded()) {
                if let resolver::BindingStatus::Excluded(reason) = &method.status {
                    println!("      ✗ {} - {:?}", method.cpp_name, reason);
                }
            }

            // Static methods
            let statics = table.included_static_methods(class);
            let all_statics: Vec<_> = class.static_methods.iter()
                .filter_map(|id| table.static_methods.get(id))
                .collect();
            if !all_statics.is_empty() {
                println!("    Static methods: {}/{} included", statics.len(), all_statics.len());
            }

            println!();
        }

        // Dump functions in this module
        let functions = table.functions_for_module(module);
        if !functions.is_empty() {
            println!("  FUNCTIONS:");
            for func in functions {
                let status_str = match &func.status {
                    resolver::BindingStatus::Included => "✓".to_string(),
                    resolver::BindingStatus::Excluded(reason) => format!("✗ {:?}", reason),
                };
                println!("    {} {} [{}]", status_str, func.cpp_name,
                    if func.status.is_included() { "included" } else { "excluded" });
            }
            println!();
        }

        // Dump enums in this module
        let enums = table.enums_for_module(module);
        if !enums.is_empty() {
            println!("  ENUMS (all excluded - externs require integer conversion):");
            for enum_decl in enums {
                println!("    ✗ {} ({} variants)", enum_decl.cpp_name, enum_decl.variants.len());
            }
            println!();
        }
    }

    println!("===== END SYMBOL TABLE DUMP =====");
}

/// Gener
... (truncated, 10661 chars)
```
</details>

<details>
<summary>Searched for text <code>handle_able_classes</code> (<code>**/resolver.rs</code>), 15 results</summary>

- [resolver.rs:354](../crates/opencascade-binding-generator/src/resolver.rs#L354)
- [resolver.rs:777](../crates/opencascade-binding-generator/src/resolver.rs#L777)
- [resolver.rs:807](../crates/opencascade-binding-generator/src/resolver.rs#L807)
- [resolver.rs:810](../crates/opencascade-binding-generator/src/resolver.rs#L810)
- [resolver.rs:815](../crates/opencascade-binding-generator/src/resolver.rs#L815)
- [resolver.rs:822](../crates/opencascade-binding-generator/src/resolver.rs#L822)
- [resolver.rs:825](../crates/opencascade-binding-generator/src/resolver.rs#L825)
- [resolver.rs:829](../crates/opencascade-binding-generator/src/resolver.rs#L829)
- [resolver.rs:853](../crates/opencascade-binding-generator/src/resolver.rs#L853)
- [resolver.rs:927](../crates/opencascade-binding-generator/src/resolver.rs#L927)
- [resolver.rs:1206](../crates/opencascade-binding-generator/src/resolver.rs#L1206)
- [resolver.rs:1248](../crates/opencascade-binding-generator/src/resolver.rs#L1248)
- [resolver.rs:1289](../crates/opencascade-binding-generator/src/resolver.rs#L1289)
- [resolver.rs:1306](../crates/opencascade-binding-generator/src/resolver.rs#L1306)
</details>

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01AHUcMcJ3KvGJfDWWL4GX9C__vscode-1771312782382/content.txt#1-1), lines 1 to 200

Read [content.txt](/Users/sethml/Library/Application Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01M1WvxkbpxaXPpgfBWMyAVE__vscode-1771312782383/content.txt#1-1), lines 1 to 250

<details>
<summary>Reading lines 770-800 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L770-L801">resolver.rs</a></summary>

```
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Compute handle-able classes (inherit from Standard_Transient and no protected destructor)
    let mut handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms)
                if !name.contains('<') && !name.contains("::") {
                    set.insert(name.clone());
                }
            }
            crate::model::Type::ConstRef(inner)
            | crate::model::Type::MutRef(inner)
            | crate::model::Type::ConstPtr(inner)
            | crate::model::Type::MutPtr(inner)
            | crate::model::Type::RValueRef(inner) => {
                collect_handle_types(inner, set);
            }
```
</details>

Searched for text `fn compute_class_bindings` (`**/codegen/bindings.rs`), 1 result
- [bindings.rs:1577](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1577)

<details>
<summary>Reading lines 1577-1650 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1577-L1651">bindings.rs</a></summary>

```
pub fn compute_class_bindings(
    class: &ParsedClass,
    ffi_ctx: &TypeContext,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
    all_classes_by_name: &HashMap<String, &ParsedClass>,
    ncollection_element_types: &HashMap<String, String>,
    ncollection_primitive_classes: &HashSet<String>,
    reexport_ctx: Option<&ReexportTypeContext>,
    exclude_methods: &HashSet<(String, String)>,
) -> ClassBindings {
    // Flatten C++ nested class names (e.g., "Parent::Child" -> "Parent_Child")
    // for use as valid Rust identifiers in ffi.rs
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;
    let all_enum_names = ffi_ctx.all_enums;

    let effectively_abstract = is_effectively_abstract(class, all_classes_by_name, symbol_table);

    // ── Constructors ────────────────────────────────────────────────────
    let exclude_ctors = exclude_methods.contains(&(class.name.clone(), class.name.clone()))
        || exclude_methods.contains(&(class.name.clone(), "*".to_string()));
    let constructors = if !effectively_abstract && !class.has_protected_destructor {
        let mut ctors = compute_constructor_bindings(class, ffi_ctx, handle_able_classes, ncollection_element_types, reexport_ctx);
        if exclude_ctors {
            ctors.clear();
        }
        // If no bindable constructors AND no explicit constructors at all,
        // generate a synthetic default constructor (uses C++ implicit default).
        // We must NOT generate synthetic constructors when:
        // - The class has explicit constructors (even if filtered out) — C++ won't
        //   generate an implicit default constructor in that case
        if ctors.is_empty() && !class.has_explicit_constructors {
            ctors.push(ConstructorBinding {
                ffi_fn_name: format!("{}_ctor", cpp_name),
                impl_method_name: "new".to_string(),
                params: Vec::new(),
                cpp_arg_exprs: Vec::new(),
                doc_comment: Some("Default constructor".to_string()),
                source_line: None,
                convenience_of: None,
            });
        }
        ctors
    } else {
        Vec::new()
    };

    // ── Direct methods (extern "C" self-pointer wrappers) ──────────────────
    let direct_methods_raw: Vec<&Method> = class
        .methods
        .iter()
        .filter(|m| !exclude_methods.contains(&(class.name.clone(), m.name.clone())))
        .filter(|m| is_method_bindable(m, ffi_ctx, cpp_name) && !needs_wrapper_function(m, all_enum_names))
        .filter(|m| !method_has_misresolved_element_type(&m.params, m.return_type.as_ref(), cpp_name, ncollection_primitive_classes))
        .collect();

    let direct_method_names = compute_direct_method_names(&direct_methods_raw);
    let direct_methods: Vec<DirectMethodBinding> = direct_methods_raw
        .iter()
        .zip(direct_method_names.iter())
        .map(|(method, rust_name)| {
            let params: Vec<ParamBinding> = method
                .params
                .iter()
                .map(|p| build_param_binding(&p.name, &p.ty, p.is_nullable_ptr(), ffi_ctx, reexport_ctx))
                .collect();
            let return_type = method
                .return_type
                .as_ref()
                .map(|ty| build_return_type_binding(ty, ffi_ctx, reexport_ctx));

            DirectMethodBinding {
                rust_name: rust_name.clone(),
```
</details>

Searched for text `compute_inherited_method_bindings(class` (`**/codegen/bindings.rs`), 1 result
- [bindings.rs:1823](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1823)

<details>
<summary>Reading lines 1800-1920 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1800-L1921">bindings.rs</a></summary>

```

    // ── to_handle ───────────────────────────────────────────────────────
    let has_to_handle =
        class.is_handle_type && !class.has_protected_destructor && !effectively_abstract;

    // ── Handle get/get_mut (works for abstract classes too) ─────────────
    let has_handle_get = class.is_handle_type && !class.has_protected_destructor;

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_handle_get {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };
    // ── Handle downcasts ─────────────────────────────────────────────
    let handle_downcasts = if has_handle_get {
        compute_handle_downcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
    };

    // ── Inherited methods ───────────────────────────────────────────────
    let inherited_methods_raw =
        compute_inherited_method_bindings(class, symbol_table, handle_able_classes, ffi_ctx.all_classes, ffi_ctx.all_enums, ncollection_primitive_classes, ffi_ctx.deletable_class_names, reexport_ctx);
    // Filter out inherited methods whose Rust name conflicts with a constructor or direct method
    let ctor_and_method_names: std::collections::HashSet<&str> = constructors
        .iter()
        .map(|c| c.impl_method_name.as_str())
        .chain(direct_methods.iter().map(|m| m.rust_name.as_str()))
        .chain(wrapper_methods.iter().map(|m| m.impl_method_name.as_str()))
        .chain(static_methods.iter().map(|m| m.impl_method_name.as_str()))
        .collect();
    let mut inherited_methods: Vec<InheritedMethodBinding> = inherited_methods_raw
        .into_iter()
        .filter(|im| !ctor_and_method_names.contains(im.impl_method_name.as_str()))
        .collect();
    // Dedup inherited methods against each other (different C++ names that
    // produce the same snake_case, e.g. GetChildLabel/getChildLabel).
    // Also dedup the FFI function names (C++ wrappers) to avoid link-time
    // collisions.
    {
        let mut seen_impl: HashMap<String, usize> = HashMap::new();
        let mut seen_ffi: HashMap<String, usize> = HashMap::new();
        for im in &mut inherited_methods {
            let impl_count = seen_impl.entry(im.impl_method_name.clone()).or_insert(0);
            *impl_count += 1;
            if *impl_count > 1 {
                im.impl_method_name = format!("{}_{}", im.impl_method_name, impl_count);
            }
            let ffi_count = seen_ffi.entry(im.ffi_fn_name.clone()).or_insert(0);
            *ffi_count += 1;
            if *ffi_count > 1 {
                im.ffi_fn_name = format!("{}_{}", im.ffi_fn_name, ffi_count);
            }
        }
    }
    // ── POD struct fields ────────────────────────────────────────────────
    let pod_fields = if class.is_pod_struct {
        compute_pod_field_bindings(&class.fields)
    } else {
        Vec::new()
    };

    ClassBindings {
        cpp_name: cpp_name.clone(),
        cpp_qualified_name: class.name.clone(),
        short_name: crate::type_mapping::safe_short_name(&crate::type_mapping::short_name_for_module(cpp_name, &class.module)),
        module: class.module.clone(),
        is_abstract: effectively_abstract,
        is_handle_type: class.is_handle_type,
        has_protected_destructor: class.has_protected_destructor,
        doc_comment: class.comment.clone(),
        source_header: class.source_header.clone(),
        source_line: class.source_line,
        constructors,
        direct_methods,
        wrapper_methods,
        static_methods,
        upcasts,
        has_to_owned,
        has_to_handle,
        has_handle_get,
        handle_upcasts,
        handle_downcasts,
        inherited_methods,
        is_pod_struct: class.is_pod_struct,
        pod_fields,
    }
}

// ── POD struct field bindings ───────────────────────────────────────────────

/// Map a ParsedField's Type to the Rust type string for
... (truncated, 5000 chars)
```
</details>

Searched for text `destructor.*emit_cpp` (`**/codegen/bindings.rs`), no results

<details>
<summary>Searched for text <code>destructor</code> (<code>**/codegen/cpp.rs</code>), 8 results</summary>

- [cpp.rs:21](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L21)
- [cpp.rs:278](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L278)
- [cpp.rs:279](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L279)
- [cpp.rs:283](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L283)
- [cpp.rs:293](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L293)
- [cpp.rs:302](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L302)
- [cpp.rs:305](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L305)
- [cpp.rs:309](../crates/opencascade-binding-generator/src/codegen/cpp.rs#L309)
</details>

<details>
<summary>Searched for text <code>_destructor</code> (<code>**/codegen/bindings.rs</code>), 30 results</summary>

- [bindings.rs:40](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L40)
- [bindings.rs:915](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L915)
- [bindings.rs:1599](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1599)
- [bindings.rs:1798](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1798)
- [bindings.rs:1803](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1803)
- [bindings.rs:1806](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1806)
- [bindings.rs:1870](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1870)
- [bindings.rs:2304](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2304)
- [bindings.rs:2312](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2312)
- [bindings.rs:2354](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2354)
- [bindings.rs:2364](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2364)
- [bindings.rs:2398](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2398)
- [bindings.rs:2408](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2408)
- [bindings.rs:2448](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2448)
- [bindings.rs:2759](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2759)
- [bindings.rs:2796](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2796)
- [bindings.rs:2921](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2921)
- [bindings.rs:2945](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2945)
- [bindings.rs:3150](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L3150)
- [bindings.rs:4209](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4209)
- [bindings.rs:4212](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4212)
- [bindings.rs:4389](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4389)
- [bindings.rs:4396](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4396)
- [bindings.rs:4398](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4398)
- [bindings.rs:4749](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4749)
- [bindings.rs:4835](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4835)
</details>

<details>
<summary>Reading lines 4200-4230 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4200-L4231">bindings.rs</a></summary>

```
                )
                .unwrap();
            }
        }

        writeln!(output, "}}").unwrap();
    }

    // 11. Destructor wrapper
    if !bindings.has_protected_destructor {
        writeln!(
            output,
            "extern \"C\" void {ffi_cn}_destructor({cn}* self_) {{ delete self_; }}"
        )
        .unwrap();
    }

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
/// Convert a param argument for FFI call: add `.into()` if it's a value enum.
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
```
</details>

> 💭 Now I have a very clear picture. Let me summarize the plan:
> 
> 1. **TDF_LabelNode**: Not useful to bind (truly internal, no public APIs expose it). Keep excluded.
> 
> 2. **The real fix**: The blanket-skip of `has_protected_destructor` classes is too aggressive. We need to:
>    - Allow method/static/upcast bindings for protected-destructor classes (no ctors or dtors)
>    - Allow Handle generation for handle types with protected destructors (Handle delete is safe)
>    
> 3. **Impact**: 
>    - Handle types like BRepMesh_IncrementalMesh, TObj_Model get Handle support
>    - Abstract bases like BRepAlgoAPI_Algo get their methods bound, though concrete subclasses already inherit them anyway
> 
> The inherited methods are already flowing to concrete subclasses, so the real wins are Handle type support and direct method access on abstract bases. For Handle types the fix is straightforward and valuable, but for abstract bases it's less critical since their methods are already available through concrete subclasses.
> 
> Let me double-check the actual situation though. If BRepAlgoAPI_Algo has methods that aren't overridden by subclasses, those would still need direct binding. And even if the class has a protected destructor, the opaque struct is still generated in ffi.rs, so you can get a reference to it. But I need to look at what's actually happening in cpp.rs around line 293 where there's a condition checking bindings.
> 
> has_protected_destructor that skips the entire C++ wrapper generation, and similarly in rust.rs at line 153 where FFI declarations are filtered out for protected-destructor classes. That means no method wrappers exist at all for these classes, even though the opaque type does. The inherited methods work on concrete subclasses because they're generated as methods of those concrete classes with their own self pointers. So the real fix is to stop skipping method generation entirely for protected-destructor classes—just skip the destructor itself—and allow Handle generation for these types. That should make everything work.
> 
> Now I'm tracing through the actual code flow more carefully. The `emit_cpp_class` and `emit_ffi_class` functions in bindings.rs handle per-class code generation, and I need to see what they actually produce. Looking at the ClassBindings struct, constructors are already empty for protected-destructor classes, and the handle-related flags are already false. So if I remove the blanket skip in cpp.rs and rust.rs, the emission functions should still work correctly since those flags are already set appropriately.

Now I have a clear picture. Let me understand the exact code that needs to change by reading the emit functions:

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- ✅ Check for other excluded non-constructible classes
- 🔄 Implement non-constructible class support
- ⬜ Test and validate changes
- ⬜ Commit changes

Let me read the key parts of the codegen to understand the destructor/CppDeletable/Handle generation:

<details>
<summary>Reading lines 4190-4250 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4190-L4251">bindings.rs</a></summary>

```
                    output,
                    "    return self->{method}({args_str});",
                    method = im.cpp_method_name
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "    self->{method}({args_str});",
                    method = im.cpp_method_name
                )
                .unwrap();
            }
        }

        writeln!(output, "}}").unwrap();
    }

    // 11. Destructor wrapper
    if !bindings.has_protected_destructor {
        writeln!(
            output,
            "extern \"C\" void {ffi_cn}_destructor({cn}* self_) {{ delete self_; }}"
        )
        .unwrap();
    }

    writeln!(output).unwrap();

    output
}

/// Emit a per-module re-export for a single class from pre-computed ClassBindings.
///
/// Produces the `pub use crate::ffi::X as ShortName;` line and the `impl ShortName { ... }`
/// block with constructor, wrapper, static, upcast, to_owned, and to_handle methods.
/// Convert a param argument for FFI call: add `.into()` if it's a value enum.
fn convert_arg(p: &ParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", p.rust_name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", p.rust_name)
        }
    } else if p.mut_ref_enum_rust_type.is_some() {
        format!("&mut {}_i32_", p.rust_name)
    } else if p.rust_reexport_type == "&str" {
        format!("c_{}.as_ptr()", p.rust_name)
    } else if p.enum_rust_type.is_some() {
        format!("{}.into()", p.rust_name)
    } else {
        p.rust_name.clone()
    }
}

fn convert_arg_resolved(name: &str, p: &ResolvedParamBinding) -> String {
    if p.is_nullable_ptr {
        if p.rust_ffi_type.starts_with("*const") {
            format!("{}.map_or(std::ptr::null(), |r| r as *const _)", name)
        } else {
            format!("{}.map_or(std::ptr::null_mut(), |r| r as *mut _)", name)
```
</details>

<details>
<summary>Reading lines 4370-4430 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4370-L4431">bindings.rs</a></summary>

```
            let trimmed = line.trim();
            if trimmed.is_empty() {
                output.push_str("///\n");
            } else {
                output.push_str(&format!("/// {}\n", trimmed));
            }
        }
    }

    // Type alias re-export
    output.push_str(&format!(
        "pub use crate::ffi::{} as {};\n\n",
        cn, short_name
    ));

    // POD structs are Copy types with real fields.
    // They still need CppDeletable because other classes may return them by pointer.
    if bindings.is_pod_struct {
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            short_name, cn
        ));
        return output;
    }

    // CppDeletable impl (unless protected destructor)
    if !bindings.has_protected_destructor {
        output.push_str(&format!(
            "unsafe impl crate::CppDeletable for {} {{\n    unsafe fn cpp_delete(ptr: *mut Self) {{\n        crate::ffi::{}_destructor(ptr);\n    }}\n}}\n\n",
            short_name, cn
        ));
    }

    // Build impl methods
    let mut impl_methods: Vec<String> = Vec::new();

    // 1. Constructors
    for ctor in &bindings.constructors {
        let params: Vec<String> = ctor
            .params
            .iter()
            .map(|p| format!("{}: {}", p.rust_name, p.rust_reexport_type))
            .collect();
        let args: Vec<String> = ctor.params.iter().map(|p| convert_arg(p)).collect();

        let source_attr = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
        let doc = format_reexport_doc(&source_attr, &ctor.doc_comment);

        if let Some(ref conv) = ctor.convenience_of {
            // Convenience constructor: Rust-only wrapper that delegates to full-arg version
            // Use raw param names (no CString conversion) since the target method handles it
            let convenience_args: Vec<String> = ctor.params.iter().map(|p| p.rust_name.clone()).collect();
            let mut all_args = convenience_args;
            all_args.extend(conv.default_exprs.iter().cloned());
            impl_methods.push(format!(
                "{}    pub fn {}({}) -> crate::OwnedPtr<Self> {{\n        Self::{}({})\n    }}\n",
                doc,
```
</details>

<details>
<summary>Reading lines 4920-5000 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4920-L5001">bindings.rs</a></summary>

```

        let params_str = format_params(&sm.params);
        let ret = if let Some(ref rt) = sm.return_type {
            format!(" -> {}", rt.rust_ffi_type)
        } else {
            String::new()
        };
        writeln!(out, "    pub fn {}({}){};", sm.ffi_fn_name, params_str, ret).unwrap();
    }

    // ── Upcasts ─────────────────────────────────────────────────────────
    for up in &bindings.upcasts {
        writeln!(out, "    /// Upcast {} to {}", cn, up.base_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *const {}) -> *const {};", up.ffi_fn_name, cn, up.base_class).unwrap();
        writeln!(out, "    /// Upcast {} to {} (mutable)", cn, up.base_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *mut {}) -> *mut {};", up.ffi_fn_name_mut, cn, up.base_class).unwrap();
    }

    // ── to_owned ────────────────────────────────────────────────────────
    if bindings.has_to_owned {
        writeln!(out, "    /// Clone via copy constructor").unwrap();
        writeln!(out, "    pub fn {}_to_owned(self_: *const {}) -> *mut {};", cn, cn, cn).unwrap();
    }

    // ── to_handle ───────────────────────────────────────────────────────
    if bindings.has_to_handle {
        let handle_type_name = format!("Handle{}", cn.replace('_', ""));
        writeln!(out, "    /// Wrap {} in a Handle", cn).unwrap();
        writeln!(out, "    pub fn {}_to_handle(obj: *mut {}) -> *mut {};", cn, cn, handle_type_name).unwrap();
    }

    // ── Handle get (dereference) ─────────────────────────────────────────
    if bindings.has_handle_get {
        let handle_type_name = format!("Handle{}", cn.replace('_', ""));
        writeln!(out, "    /// Destroy Handle<{}>", cn).unwrap();
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", handle_type_name, handle_type_name).unwrap();
        writeln!(out, "    /// Dereference Handle to get *const {}", cn).unwrap();
        writeln!(out, "    pub fn {}_get(handle: *const {}) -> *const {};", handle_type_name, handle_type_name, cn).unwrap();
        writeln!(out, "    /// Dereference Handle to get *mut {}", cn).unwrap();
        writeln!(out, "    pub fn {}_get_mut(handle: *mut {}) -> *mut {};", handle_type_name, handle_type_name, cn).unwrap();
    }

    // ── Handle upcasts ──────────────────────────────────────────────────
    for hu in &bindings.handle_upcasts {
        writeln!(out, "    /// Upcast Handle<{}> to Handle<{}>", cn, hu.base_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *const {}) -> *mut {};", hu.ffi_fn_name, hu.derived_handle_name, hu.base_handle_name).unwrap();
    }

    // ── Handle downcasts ─────────────────────────────────────────────────────
    for hd in &bindings.handle_downcasts {
        writeln!(out, "    /// Downcast Handle<{}> to Handle<{}> (returns null on failure)", cn, hd.derived_class).unwrap();
        writeln!(out, "    pub fn {}(self_: *const {}) -> *mut {};", hd.ffi_fn_name, hd.base_handle_name, hd.derived_handle_name).unwrap();
    }

    // ── Inherited methods (free functions with self_ parameter) ─────────
    for im in &bindings.inherited_methods {
        let source = format_source_attribution(
            &im.source_header,
            im.source_line,
            &format!("{}::{}()", im.source_class, im.cpp_method_name),
        );
        writeln!(out, "    /// Inherited: {}", source).unwrap();

        let self_param = if im.is_const {
            format!("self_: *const {}", cn)
        } else {
            format!("self_: *mut {}", cn)
        };
        let params_str: String = im
            .params
            .iter()
            .map(|p| format!("{}: {}", safe_param_name(&p.rust_name), p.rust_ffi_type))
            .collect::<Vec<_>>()
            .join(", ");
        let all_params = if params_str.is_empty() {
            self_param
        } else {
            format!("{}, {}", self_param, params_str)
        };
        let ret = im.return_type.as_ref()
            .map(|rt| format!(" -> {
... (truncated, 4022 chars)
```
</details>

Now let me check where handle_able_classes is computed and where Handle types are collected for generation:

<details>
<summary>Reading lines 2750-2820 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2750-L2821">bindings.rs</a></summary>

```
    extra_typedef_names: &HashSet<String>,
    exclude_methods: &HashSet<(String, String)>,
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the 91 manually-specified known collections (which get generated destructors).
    // NCollection typedef names from extra_typedef_names are NOT included here.
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for class in all_classes {
        for method in &class.methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
        for method in &class.static_methods {
            if let Some(ref ret) = method.return_type {
                collect_nested_deletable_names(ret, &known_class_names, &mut deletable_class_names);
            }
            for param in &method.params {
                collect_nested_deletable_names(&param.ty, &known_class_names, &mut deletable_class_names);
            }
        }
    }

    // Full known-type set (for param filtering): adds NCollection template typedefs
    // so methods passing them as params pass the unknown-type filter.
    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
    };

    let all_classes_by_name: HashMap<String, &ParsedClass> = all_classes
        .iter()
        .map(|c| (c.name.clone(), *c))
        .collect();

    // Build NCollection class element type maps for misresolution detection
    let (ncollection_element_types, ncollection_primitive_classes) = build_ncollection_element_types(all_classes);

    let class_public_info = build_class_public_info(all_classes);

```
</details>

<details>
<summary>Reading lines 1-50 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L1-L51">cpp.rs</a></summary>

```
//! C++ wrapper code generation
//!
//! Generates extern "C" wrapper functions for all OCCT methods:
//! - Constructors (using placement new or heap allocation)
//! - Return-by-value methods
//! - Static methods
//! - Overloaded methods
//!
//! All wrapper functions use extern "C" linkage for direct FFI access.

use crate::model::{ParsedClass, Type};
use crate::resolver::SymbolTable;
use std::collections::HashSet;
use std::fmt::Write;

fn collect_handle_types(classes: &[&ParsedClass]) -> Vec<(String, String)> {
    let mut handles = HashSet::new();

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
        }

        for method in &class.methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for method in &class.static_methods {
            collect_type_handles(&method.return_type, &mut handles);
            for param in &method.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }

        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_type_handles(&Some(param.ty.clone()), &mut handles);
            }
        }
    }

    let mut result: Vec<_> = handles
        .into_iter()
        .filter(|inner_class| {
            // Skip namespace-scoped types (e.g., IMeshData::ListOfPnt2d) and pointer types
            !inner_class.contains("::") && !inner_class.contains('*') && !inner_class.contains('&')
```
</details>

Now let me read the rust.rs more to see how handle declarations and module re-exports handle protected destructor filtering:

<details>
<summary>Reading lines 20-55 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L20-L56">rust.rs</a></summary>

```
    /// Class types (e.g., "gp_Pnt", "Geom_TrimmedCurve") - sorted for deterministic output
    pub classes: BTreeSet<String>,
    /// Handle types with their inner class (e.g., "Geom_TrimmedCurve" for Handle<Geom_TrimmedCurve>) - sorted for deterministic output
    pub handles: BTreeSet<String>,
}

/// Collect all referenced OCCT types from class methods and constructors
pub fn collect_referenced_types(
    classes: &[&ParsedClass],
) -> CollectedTypes {
    let mut result = CollectedTypes {
        classes: BTreeSet::new(),
        handles: BTreeSet::new(),
    };

    for class in classes {
        // Add Handle type for classes that are transient (can be wrapped in Handle)
        // This ensures the Handle type is declared even if not used in method signatures
        if class.is_handle_type && !class.has_protected_destructor {
            result.handles.insert(class.name.clone());
        }

        // From constructors
        for ctor in &class.constructors {
            for param in &ctor.params {
                collect_types_from_type(&param.ty, &mut result);
            }
        }

        // From methods
        for method in &class.methods {
            for param in &method.params {
                collect_types_from_type(&param.ty, &mut result);
            }
            if let Some(ref ret) = method.return_type {
                collect_types_from_type(ret, &mut result);
```
</details>

<details>
<summary>Reading lines 380-430 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L380-L431">rust.rs</a></summary>

```
            .map(|rt| format!(" -> {}", rt.rust_ffi_type))
            .unwrap_or_default();

        let source_attr = format_source_attribution(
            &func.source_header,
            func.source_line,
            &format!("{}::{}", func.namespace, func.short_name),
        );
        writeln!(out, "    /// {}", source_attr).unwrap();
        writeln!(out, "    pub fn {}({}){};\n", func.cpp_wrapper_name, params_str, ret_str).unwrap();
    }
    out
}

/// Generate Handle type declarations
fn generate_handle_declarations(classes: &[&ParsedClass], extra_handle_able: &HashSet<String>) -> String {
    let mut handles = BTreeSet::new();

    // Classes parsed from non-excluded headers
    let mut defined_handles = BTreeSet::new();
    for class in classes {
        if class.is_handle_type && !class.has_protected_destructor {
            handles.insert(class.name.clone());
            defined_handles.insert(class.name.clone());
        }
    }

    // Also generate Handle declarations for types that appear in Handle(...)
    // in method signatures, even if their own headers are excluded.
    // This ensures that methods like GeomAPI_Interpolate(Handle(TColgp_HArray1OfPnt))
    // can be generated even when TColgp_HArray1OfPnt.hxx is excluded.
    for name in extra_handle_able {
        handles.insert(name.clone());
    }

    let mut out = String::new();
    for class_name in &handles {
        let handle_type_name = format!("Handle{}", class_name.replace('_', ""));
        writeln!(out, "/// Handle to {}", class_name).unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", handle_type_name).unwrap();
    }

    // For extra handle types (not from parsed classes), generate standalone
    // CppDeletable impls and destructor FFI declarations.
    // Parsed classes get these in their module files instead.
    let extra_handles: Vec<_> = handles.iter()
        .filter(|name| !defined_handles.contains(*name))
        .collect();
    if !extra_handles.is_empty() {
        writeln!(out).unwrap();
```
</details>

<details>
<summary>Reading lines 780-850 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L780-L851">rust.rs</a></summary>

```
    if !collections.is_empty() {
        output.push('\n');
    }

    // Generate impl blocks for collection types
    for coll in collections {
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
        }
        for hd in &b.handle_downcasts {
            if !directly_exported_handles.contains(&hd.derived_handle_name) {
                base_handle_reexports.insert(hd.derived_handle_name.clone());
            }
        }
    }

    // Emit base handle type re-exports at the top of the module
    if !base_handle_reexports.is_empty() {
        output.push_str("// Handle type re-exports (targets of handle upcasts/downcasts)\n");
        for handle_name in &base_handle_reexports {
            output.push_str(&format!("pub use crate::ffi::{};\n", handle_name));
        }
        output.push_str("\n");
    }

    for (header, header_bindings) in bindings_by_header {
        // Output section header
        output.push_str("// ========================\n");
        output.push_str(&format!("// From {}\n", header));
        output.push_str("// ========================\n\n");
```
</details>

Now let me also check the `crates/opencascade-binding-generator/src/main.rs` for Handle FFI type tracking:

<details>
<summary>Reading lines 640-700 of <a href="../crates/opencascade-binding-generator/src/main.rs#L640-L701">main.rs</a></summary>

```
            .entry(fb.module.clone())
            .or_default()
            .push(fb);
    }

    // Compute ALL types that appear in ffi.rs so we can find unreexported ones
    // 1. Class types from ClassBindings (already re-exported via emit_reexport_class)
    let mut already_reexported: HashSet<String> = HashSet::new();
    for b in &all_bindings {
        if !b.has_protected_destructor {
            already_reexported.insert(b.cpp_name.clone());
            // Handle types generated for this class
            if b.has_to_handle {
                let handle_name = format!("Handle{}", b.cpp_name.replace('_', ""));
                already_reexported.insert(handle_name);
            }
            // Handle upcasts reference base handle types
            for hu in &b.handle_upcasts {
                already_reexported.insert(hu.base_handle_name.clone());
            }
        }
    }

    // 2. Collection types (re-exported via collections loop)
    for coll in &all_collections {
        already_reexported.insert(coll.typedef_name.clone());
    }

    // Now compute ALL types in ffi.rs and find unreexported ones:
    // A. Handle types for all transient classes
    let mut all_ffi_types: Vec<(String, String)> = Vec::new(); // (ffi_name, module_prefix)
    for class in all_classes {
        if class.is_handle_type && !class.has_protected_destructor {
            let handle_name = format!("Handle{}", class.name.replace('_', ""));
            if !already_reexported.contains(&handle_name) {
                // Use the class's actual module (not derived from handle name)
                all_ffi_types.push((handle_name, class.module.clone()));
            }
        }
    }

    // B. Opaque referenced types (types referenced in method signatures but not defined)
    let collected_types = codegen::rust::collect_referenced_types(all_classes);
    let defined_classes: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    let all_enum_names = &symbol_table.all_enum_names;

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) { continue; }
        if all_enum_names.contains(type_name) { continue; }
        // Protected destructor classes are still needed as type references in module reexports
        if codegen::rust::is_primitive_type(type_name) { continue; }
        if collection_type_names.contains(type_name) { continue; }
        if already_reexported.contains(type_name) { continue; }
        // Skip namespace-scoped types and types with pointer/ref qualifiers
        // leaked into the name (e.g., "IMeshData::ListOfPnt2d", "IMeshData_Edge *const")
        if type_name.contains("::") || type_name.contains('*') || type_name.contains('&') { continue; }

        // Determine module from type_to_module map, falling back to name-based
        if let Some(module) = symbol_table.type_to_module.get(type_name) {
            all_ffi_types.push((type_name.clone(), module.clone()));
        } else if let Some(underscore_pos) = type_name.find('_') {
```
</details>

Now I have the complete picture. Let me also check `crates/opencascade-binding-generator/src/resolver.rs` for `handle_able_classes`:

<details>
<summary>Reading lines 770-790 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L770-L791">resolver.rs</a></summary>

```
    // Collect all enum and class names first
    let all_enum_names: HashSet<String> = all_enums.iter().map(|e| e.name.clone()).collect();
    let mut all_class_names: HashSet<String> = all_classes.iter().map(|c| c.name.clone()).collect();
    // Collection typedefs are known types for filtering purposes
    all_class_names.extend(collection_type_names.iter().cloned());

    // Compute handle-able classes (inherit from Standard_Transient and no protected destructor)
    let mut handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
    fn collect_handle_types(ty: &crate::model::Type, set: &mut HashSet<String>) {
        match ty {
            crate::model::Type::Handle(name) => {
                // Only add clean OCCT type names (not template forms)
                if !name.contains('<') && !name.contains("::") {
```
</details>

Now I have a complete understanding. Let me plan the implementation:

**Summary of the problem:**
-  acts as a blanket kill-switch that skips ALL codegen for a class
- This prevents Handle generation for handle types (Handle manages lifetime safely)
- This prevents method/static/upcast generation for abstract bases (methods are useful through concrete subclasses)
- **However**, inherited methods DO already get copied to concrete subclasses (so BRepAlgoAPI_Cut already gets Shape())

**The changes needed:**

1. **`crates/opencascade-binding-generator/src/codegen/bindings.rs`**: Remove  filter from `handle_able_classes`, `build_class_public_info`, and allow `has_to_handle`/`has_handle_get` for handle types with protected dtors
2. **cpp.rs**: Remove blanket skip; destructor skip is already handled inside `emit_cpp_class`
3. **rust.rs**: Remove blanket skip in multiple places; Handle generation for handle types with protected dtors; module re-export generation
4. **`crates/opencascade-binding-generator/src/resolver.rs`**: Allow handle types with protected dtors in `handle_able_classes`
5. **`crates/opencascade-binding-generator/src/main.rs`**: Allow handle types with protected dtors in tracking

Let me now implement all these changes:    Now let me start the implementation. First, let me re-read all the exact lines I need to edit:

<details>
<summary>Reading lines 288-300 of <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L288-L301">cpp.rs</a></summary>

```
        writeln!(output).unwrap();
    }

    // Generate wrapper functions for ALL classes from pre-computed ClassBindings
    for bindings in all_bindings {
        if bindings.has_protected_destructor {
            continue;
        }
        output.push_str(&super::bindings::emit_cpp_class(bindings));
    }

    // Generate wrappers for ALL namespace-level free functions
    generate_function_wrappers(&mut output, function_bindings, known_headers);
```
</details>

Now let me implement all the changes. I'll make the edits in a systematic order:

**cpp.rs**: Remove blanket skip in wrapper generation (line 293-295) and handle type collection (line 21)
**rust.rs**: Remove blanket skip in FFI declarations (line 153), reference type collection (line 38), handle declarations (line 401), module re-exports (lines 794, 808, 822)
**`crates/opencascade-binding-generator/src/codegen/bindings.rs`**: Allow handle types with protected dtors in `handle_able_classes` (line 2796), `build_class_public_info` (line 915), `has_to_handle` (line 1803), `has_handle_get` (line 1806)
**`crates/opencascade-binding-generator/src/resolver.rs`**: Allow handle types with protected dtors (line 779)
**`crates/opencascade-binding-generator/src/main.rs`**: Allow handle types with protected dtors (line 649, 672)

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L20-L26">cpp.rs -4+6</a>, <a href="../crates/opencascade-binding-generator/src/codegen/cpp.rs#L293-L297">-7+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "291:tb,292:ie,293:uc,294:zr,295:cy,296:wp,297:ax",
      "content": "    // Generate wrapper functions for ALL classes from pre-computed ClassBindings\n    for bindings in all_bindings {\n        output.push_str(&super::bindings::emit_cpp_class(bindings));\n    }"
    },
    {
      "filePath": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "lineHashes": "20:sf,21:tj,22:sc,23:cy",
      "content": "        // Add Handle type for classes that are transient (can be wrapped in Handle)\n        // Handle types with protected destructors are included because Handle<T>\n        // manages lifetime via reference counting, not direct delete.\n        if class.is_handle_type {\n            handles.insert(class.name.clone());\n        }"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/cpp.rs",
      "startLine": 20,
      "endLine": 296
    }
  ]
}
```
</details>

<details>
<summary>Reading lines 148-163 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L148-L164">rust.rs</a></summary>

```
        .collect();

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: String = all_bindings
        .iter()
        .filter(|b| !b.has_protected_destructor)
        .filter(|b| !collection_type_names.contains(&b.cpp_name))
        .map(|b| super::bindings::emit_ffi_class(b))
        .collect();

    // Generate namespace-level free functions from pre-computed FunctionBindings
    let function_items = generate_functions_from_bindings(function_bindings);

    // Generate Handle type declarations
    let handle_decls = generate_handle_declarations(all_classes, &symbol_table.handle_able_classes);

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L36-L42">rust.rs -5+6</a>, <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L151-L157">-7+6</a>, <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L398-L408">-8+10</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Emit ffi declarations from pre-computed ClassBindings\n    let class_items: String = all_bindings\n        .iter()\n        .filter(|b| !collection_type_names.contains(&b.cpp_name))\n        .map(|b| super::bindings::emit_ffi_class(b))\n        .collect();",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "150:rt,151:bw,152:cb,153:qe,154:kc,155:on,156:nj"
    },
    {
      "content": "        // Add Handle type for classes that are transient (can be wrapped in Handle)\n        // Handle types with protected destructors are included because Handle<T>\n        // manages lifetime via reference counting, not direct delete.\n        if class.is_handle_type {\n            result.handles.insert(class.name.clone());\n        }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "36:sf,37:nw,38:tj,39:da,40:cy"
    },
    {
      "content": "    // Classes parsed from non-excluded headers\n    // Include handle types even with protected destructors since Handle<T>\n    // manages lifetime via reference counting, not direct delete.\n    let mut defined_handles = BTreeSet::new();\n    for class in classes {\n        if class.is_handle_type {\n            handles.insert(class.name.clone());\n            defined_handles.insert(class.name.clone());\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "398:cw,399:ix,400:co,401:tj,402:sc,403:qf,404:cy,405:ax"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "startLine": 36,
      "endLine": 407
    }
  ]
}
```
</details>

Now fix the module re-export skips in rust.rs:

<details>
<summary>Reading lines 788-830 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L788-L831">rust.rs</a></summary>

```
        output.push_str(&emit_collection_impl(coll));
    }

    // Group pre-computed bindings by source header for organized output
    use std::collections::BTreeMap;
    let mut bindings_by_header: BTreeMap<String, Vec<&super::bindings::ClassBindings>> =
        BTreeMap::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        bindings_by_header
            .entry(b.source_header.clone())
            .or_default()
            .push(b);
    }

    // Generate re-exports and impl blocks for classes, grouped by header
    // Collect all handle types that are directly re-exported (derived handles with to_handle),
    // so we can avoid duplicating their re-export when they appear as upcast targets.
    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        if b.has_to_handle || b.has_handle_get {
            let handle_type_name = format!("Handle{}", b.cpp_name.replace("_", ""));
            directly_exported_handles.insert(handle_type_name);
        }
    }

    // Also collect handle types referenced by upcast/downcast methods that need re-exporting.
    // These are handle types for base classes (upcast targets) or derived classes (downcast targets)
    // that external crates need to name.
    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for b in module_bindings {
        if b.has_protected_destructor {
            continue;
        }
        for hu in &b.handle_upcasts {
            if !directly_exported_handles.contains(&hu.base_handle_name) {
                base_handle_reexports.insert(hu.base_handle_name.clone());
            }
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L795-L801">rust.rs -9+6</a>, <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L805-L812">-10+7</a>, <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L816-L822">-9+6</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    for b in module_bindings {\n        bindings_by_header\n            .entry(b.source_header.clone())\n            .or_default()\n            .push(b);\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "795:ns,796:sa,797:zr,798:cy,799:rj,800:ze,801:zd,802:hx,803:ax"
    },
    {
      "content": "    let mut directly_exported_handles: std::collections::HashSet<String> = std::collections::HashSet::new();\n    for b in module_bindings {\n        if b.has_to_handle || b.has_handle_get {\n            let handle_type_name = format!(\"Handle{}\", b.cpp_name.replace(\"_\", \"\"));\n            directly_exported_handles.insert(handle_type_name);\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "808:iw,809:ns,810:sa,811:zr,812:cy,813:al,814:pt,815:vi,816:cy,817:ax"
    },
    {
      "content": "    let mut base_handle_reexports: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();\n    for b in module_bindings {\n        for hu in &b.handle_upcasts {\n            if !directly_exported_handles.contains(&hu.base_handle_name) {\n                base_handle_reexports.insert(hu.base_handle_name.clone());\n            }",
      "filePath": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "lineHashes": "822:dl,823:ns,824:sa,825:zr,826:cy,827:zm,828:lp,829:sj,830:uo"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/rust.rs",
      "startLine": 795,
      "endLine": 821
    }
  ]
}
```
</details>

Now fix `crates/opencascade-binding-generator/src/codegen/bindings.rs` - handle_able_classes, build_class_public_info, has_to_handle, has_handle_get:

<details>
<summary>Reading lines 910-920 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L910-L921">bindings.rs</a></summary>

```
/// Maps C++ class name → (rust_module_name, short_name).
/// Only includes classes that get `pub use` re-exports (excludes protected-destructor classes).
pub(crate) fn build_class_public_info(all_classes: &[&ParsedClass]) -> HashMap<String, (String, String)> {
    all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| {
            let ffi_name = c.name.replace("::", "_");
            let module_rust = crate::module_graph::module_to_rust_name(&c.module);
            let short = crate::type_mapping::safe_short_name(&crate::type_mapping::short_name_for_module(&ffi_name, &c.module));
            (c.name.clone(), (module_rust, short))
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L910-L915">bindings.rs -7+5</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "/// Maps C++ class name → (rust_module_name, short_name).\npub(crate) fn build_class_public_info(all_classes: &[&ParsedClass]) -> HashMap<String, (String, String)> {\n    all_classes\n        .iter()\n        .map(|c| {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "910:ut,911:vj,912:md,913:pk,914:cb,915:ct,916:rs"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":910,"endLine":914}
```
</details>

Now fix handle_able_classes in bindings.rs:

<details>
<summary>Reading lines 2793-2800 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2793-L2801">bindings.rs</a></summary>

```
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2794-L2795">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        .filter(|c| c.is_handle_type)",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2794:po"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2794,"endLine":2794}
```
</details>

Now fix `has_to_handle` and `has_handle_get` in bindings.rs:

<details>
<summary>Reading lines 1795-1810 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1795-L1811">bindings.rs</a></summary>

```
    let has_to_owned = copyable_modules.contains(&class.module.as_str())
        && !class.has_protected_destructor
        && !effectively_abstract;

    // ── to_handle ───────────────────────────────────────────────────────
    let has_to_handle =
        class.is_handle_type && !class.has_protected_destructor && !effectively_abstract;

    // ── Handle get/get_mut (works for abstract classes too) ─────────────
    let has_handle_get = class.is_handle_type && !class.has_protected_destructor;

    // ── Handle upcasts ──────────────────────────────────────────────────
    let handle_upcasts = if has_handle_get {
        compute_handle_upcast_bindings(class, symbol_table, handle_able_classes)
    } else {
        Vec::new()
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1799-L1810">bindings.rs -6+11</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // ── to_handle ───────────────────────────────────────────────────────\n    // Handle types with protected destructors can still use to_handle because\n    // Handle<T> manages lifetime via reference counting, not direct delete.\n    // However, to_handle requires constructability (it takes ownership of a raw pointer),\n    // so skip for abstract classes and classes with protected destructors.\n    let has_to_handle =\n        class.is_handle_type && !class.has_protected_destructor && !effectively_abstract;\n\n    // ── Handle get/get_mut (works for abstract classes too) ─────────────\n    // Also works for protected-destructor classes since we're just dereferencing the Handle.\n    let has_handle_get = class.is_handle_type;",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "1799:mm,1800:ke,1801:mh,1802:dd,1803:wj,1804:cr"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":1799,"endLine":1809}
```
</details>

Now fix resolver.rs:

<details>
<summary>Reading lines 775-785 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L775-L786">resolver.rs</a></summary>

```

    // Compute handle-able classes (inherit from Standard_Transient and no protected destructor)
    let mut handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    // Also add any class name that appears inside Type::Handle(...) in method signatures.
    // If C++ code uses Handle(X) for a type, X must inherit from Standard_Transient,
    // so it's handle-able even if its own header is excluded.
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L776-L784">resolver.rs -6+8</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Compute handle-able classes (inherit from Standard_Transient)\n    // Include handle types even with protected destructors because Handle<T>\n    // manages lifetime via reference counting, not direct delete.\n    let mut handle_able_classes: HashSet<String> = all_classes\n        .iter()\n        .filter(|c| c.is_handle_type)\n        .map(|c| c.name.clone())\n        .collect();",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "776:qi,777:ec,778:cb,779:po,780:hm,781:nj"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":776,"endLine":783}
```
</details>

Now fix main.rs:

<details>
<summary>Reading lines 644-680 of <a href="../crates/opencascade-binding-generator/src/main.rs#L644-L681">main.rs</a></summary>

```

    // Compute ALL types that appear in ffi.rs so we can find unreexported ones
    // 1. Class types from ClassBindings (already re-exported via emit_reexport_class)
    let mut already_reexported: HashSet<String> = HashSet::new();
    for b in &all_bindings {
        if !b.has_protected_destructor {
            already_reexported.insert(b.cpp_name.clone());
            // Handle types generated for this class
            if b.has_to_handle {
                let handle_name = format!("Handle{}", b.cpp_name.replace('_', ""));
                already_reexported.insert(handle_name);
            }
            // Handle upcasts reference base handle types
            for hu in &b.handle_upcasts {
                already_reexported.insert(hu.base_handle_name.clone());
            }
        }
    }

    // 2. Collection types (re-exported via collections loop)
    for coll in &all_collections {
        already_reexported.insert(coll.typedef_name.clone());
    }

    // Now compute ALL types in ffi.rs and find unreexported ones:
    // A. Handle types for all transient classes
    let mut all_ffi_types: Vec<(String, String)> = Vec::new(); // (ffi_name, module_prefix)
    for class in all_classes {
        if class.is_handle_type && !class.has_protected_destructor {
            let handle_name = format!("Handle{}", class.name.replace('_', ""));
            if !already_reexported.contains(&handle_name) {
                // Use the class's actual module (not derived from handle name)
                all_ffi_types.push((handle_name, class.module.clone()));
            }
        }
    }

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/main.rs#L645-L660">main.rs -17+15</a>, <a href="../crates/opencascade-binding-generator/src/main.rs#L666-L678">-12+12</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Compute ALL types that appear in ffi.rs so we can find unreexported ones\n    // 1. Class types from ClassBindings (already re-exported via emit_reexport_class)\n    let mut already_reexported: HashSet<String> = HashSet::new();\n    for b in &all_bindings {\n        already_reexported.insert(b.cpp_name.clone());\n        // Handle types generated for this class\n        if b.has_to_handle || b.has_handle_get {\n            let handle_name = format!(\"Handle{}\", b.cpp_name.replace('_', \"\"));\n            already_reexported.insert(handle_name);\n        }\n        // Handle upcasts reference base handle types\n        for hu in &b.handle_upcasts {\n            already_reexported.insert(hu.base_handle_name.clone());\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "645:ki,646:vj,647:px,648:az,649:nc,650:ye,651:yb,652:yc,653:cl,654:hc,655:uo,656:rt,657:to,658:ej,659:uo,660:cy,661:ax"
    },
    {
      "content": "    // Now compute ALL types in ffi.rs and find unreexported ones:\n    // A. Handle types for all transient classes\n    let mut all_ffi_types: Vec<(String, String)> = Vec::new(); // (ffi_name, module_prefix)\n    for class in all_classes {\n        if class.is_handle_type {\n            let handle_name = format!(\"Handle{}\", class.name.replace('_', \"\"));\n            if !already_reexported.contains(&handle_name) {\n                // Use the class's actual module (not derived from handle name)\n                all_ffi_types.push((handle_name, class.module.clone()));\n            }\n        }\n    }",
      "filePath": "crates/opencascade-binding-generator/src/main.rs",
      "lineHashes": "668:ns,669:lq,670:iv,671:yi,672:tj,673:mu,674:xb,675:zi,676:lj,677:uo,678:cy,679:ax"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/main.rs",
      "startLine": 645,
      "endLine": 677
    }
  ]
}
```
</details>

Now let me also update the `crates/opencascade-binding-generator/src/type_mapping.rs` comments:

<details>
<summary>Reading lines 288-340 of <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L288-L341">type_mapping.rs</a></summary>

```
    /// All enum names across all modules (full C++ names like "TopAbs_Orientation")
    pub all_enums: &'a std::collections::HashSet<String>,
    /// All class names across all modules (full C++ names like "gp_Pnt")
    pub all_classes: &'a std::collections::HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type && !has_protected_destructor)
    /// If None, falls back to all_classes for Handle type checking
    pub handle_able_classes: Option<&'a std::collections::HashSet<String>>,
    /// Authoritative type→module mapping (from resolver's SymbolTable)
    /// When present, used instead of name-based derivation
    pub type_to_module: Option<&'a std::collections::HashMap<String, String>>,
    /// Mapping from C++ enum name to qualified Rust enum type path.
    /// Value enums get typed Rust enums; bitset enums stay as i32.
    pub enum_rust_types: Option<&'a std::collections::HashMap<String, String>>,
    /// Class names that have `CppDeletable` impls generated (ParsedClasses without
    /// protected_destructor + the 91 manually-specified known collections).
    /// Methods returning a bare `Class(name)` value (generating `OwnedPtr<name>`) are
    /// only allowed if `name` is in this set. If `None`, no constraint is applied.
    pub deletable_class_names: Option<&'a std::collections::HashSet<String>>,
}

/// Check if a type references an unknown class/handle
/// Returns true if the type uses a Handle or Class that is not in all_classes
pub fn type_uses_unknown_class(ty: &Type, all_classes: &std::collections::HashSet<String>) -> bool {
    match ty {
        Type::Handle(class_name) => !all_classes.contains(class_name),
        Type::Class(class_name) => {
            if all_classes.contains(class_name) {
                return false;
            }
            // Nested types (Parent::Nested) are known if the parent class is known
            if let Some(parent) = class_name.split("::").next() {
                if class_name.contains("::") && all_classes.contains(parent) {
                    return false;
                }
            }
            true
        }
        Type::ConstRef(inner) | Type::MutRef(inner) => type_uses_unknown_class(inner, all_classes),
        _ => false,
    }
}

/// Check if a type references a Handle to a class that won't have a Handle declaration generated
/// This is more strict than type_uses_unknown_class - it checks that Handle types are for
/// classes that will actually have Handle<T> declarations generated (is_handle_type && !has_protected_destructor)
pub fn type_uses_unknown_handle(
    ty: &Type,
    all_classes: &std::collections::HashSet<String>,
    handle_able_classes: &std::collections::HashSet<String>,
) -> bool {
    match ty {
        Type::Handle(class_name) => !handle_able_classes.contains(class_name),
        Type::Class(class_name) => {
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L292-L294">type_mapping.rs -2+2</a>, <a href="../crates/opencascade-binding-generator/src/type_mapping.rs#L330-L333">-3+3</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    /// Classes that can have Handle<T> declarations (is_handle_type)\n    /// If None, falls back to all_classes for Handle type checking",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "292:bq,293:ya"
    },
    {
      "content": "/// Check if a type references a Handle to a class that won't have a Handle declaration generated\n/// This is more strict than type_uses_unknown_class - it checks that Handle types are for\n/// classes that will actually have Handle<T> declarations generated (is_handle_type)",
      "filePath": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "lineHashes": "330:jh,331:km,332:vh"
    }
  ]
}
```
**Output:**
```
{
  "applied": 2,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/type_mapping.rs",
      "startLine": 292,
      "endLine": 332
    }
  ]
}
```
</details>

Now let me also update the resolver comment about handle_able in the protected_destructor_class_names method and the SymbolTable comment:

<details>
<summary>Reading lines 348-360 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L348-L361">resolver.rs</a></summary>

```
    pub enums_by_module: HashMap<String, Vec<SymbolId>>,
    /// All enum names (for filtering methods that use enums)
    pub all_enum_names: HashSet<String>,
    /// All class names (including collection typedef names)
    pub all_class_names: HashSet<String>,
    /// Classes that can have Handle<T> declarations (is_handle_type && !has_protected_destructor)
    pub handle_able_classes: HashSet<String>,
    /// Cross-module type references by module
    pub cross_module_types: HashMap<String, Vec<CrossModuleType>>,
    /// Authoritative mapping from C++ type name to module name (built from parsed headers)
    /// This is the single source of truth for "which module does type X belong to?"
    pub type_to_module: HashMap<String, String>,
    /// Mapping from C++ enum name to qualified Rust enum type path (e.g., "crate::top_abs::Orientation")
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L353-L354">resolver.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    /// Classes that can have Handle<T> declarations (is_handle_type)",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "353:bq"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":353,"endLine":353}
```
</details>

Now let me check if there are other places in the codebase that check  that I might have missed:

<details>
<summary>Searched for text <code>has_protected_destructor</code> (<code>**/crates/opencascade-binding-generator/**</code>), 20 results</summary>

- [parser.rs:406](../crates/opencascade-binding-generator/src/parser.rs#L406)
- [parser.rs:413](../crates/opencascade-binding-generator/src/parser.rs#L413)
- [parser.rs:587](../crates/opencascade-binding-generator/src/parser.rs#L587)
- [resolver.rs:124](../crates/opencascade-binding-generator/src/resolver.rs#L124)
- [resolver.rs:470](../crates/opencascade-binding-generator/src/resolver.rs#L470)
- [resolver.rs:952](../crates/opencascade-binding-generator/src/resolver.rs#L952)
- [resolver.rs:1019](../crates/opencascade-binding-generator/src/resolver.rs#L1019)
- [model.rs:132](../crates/opencascade-binding-generator/src/model.rs#L132)
- [bindings.rs:40](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L40)
- [bindings.rs:1597](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1597)
- [bindings.rs:1796](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1796)
- [bindings.rs:1805](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1805)
- [bindings.rs:1873](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1873)
- [bindings.rs:2451](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2451)
- [bindings.rs:2762](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2762)
- [bindings.rs:2924](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2924)
- [bindings.rs:2948](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2948)
- [bindings.rs:4212](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4212)
</details>

There are still some references I need to check. Let me look at:
- Line 1805 (has_to_handle - already changed but let me verify)
- Line 2451 (inherited methods skip - this is correct to keep)
- Line 2762 (deletable_class_names)
- Line 2924 (another filter)
- Line 2948 (another handle filter)
- Line 4212 and 4399 (destructor and CppDeletable skip - correct to keep)

<details>
<summary>Reading lines 1800-1810 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1800-L1811">bindings.rs</a></summary>

```
    // Handle types with protected destructors can still use to_handle because
    // Handle<T> manages lifetime via reference counting, not direct delete.
    // However, to_handle requires constructability (it takes ownership of a raw pointer),
    // so skip for abstract classes and classes with protected destructors.
    let has_to_handle =
        class.is_handle_type && !class.has_protected_destructor && !effectively_abstract;

    // ── Handle get/get_mut (works for abstract classes too) ─────────────
    // Also works for protected-destructor classes since we're just dereferencing the Handle.
    let has_handle_get = class.is_handle_type;

```
</details>

<details>
<summary>Reading lines 2445-2460 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2445-L2461">bindings.rs</a></summary>

```
    all_class_names: &HashSet<String>,
    all_enum_names: &HashSet<String>,
    ncollection_primitive_classes: &HashSet<String>,
    deletable_class_names: Option<&HashSet<String>>,
    reexport_ctx: Option<&ReexportTypeContext>,
) -> Vec<InheritedMethodBinding> {
    if class.has_protected_destructor {
        return Vec::new();
    }

    let existing_method_names: HashSet<String> =
        class.methods.iter().map(|m| m.name.clone()).collect();
    let mut seen_methods: HashSet<String> = HashSet::new();
    // Methods that an intermediate ancestor has re-declared as protected/private.
    // These must not be generated as inherited bindings even if a more-distant
    // ancestor exposes them publicly (e.g. BOPAlgo_PaveFiller narrowing Clear()).
```
</details>

<details>
<summary>Reading lines 2755-2770 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2755-L2771">bindings.rs</a></summary>

```
) -> Vec<ClassBindings> {
    // Classes with CppDeletable impls: ParsedClasses (without protected dtor) +
    // the 91 manually-specified known collections (which get generated destructors).
    // NCollection typedef names from extra_typedef_names are NOT included here.
    // Nested types (Parent::Nested) get destructors generated, so include them too.
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types (those with :: in their name) as deletable
    // since we generate destructors for them
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for class in all_classes {
```
</details>

<details>
<summary>Reading lines 2920-2960 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2920-L2961">bindings.rs</a></summary>

```

    // Build TypeContext
    let mut deletable_class_names: HashSet<String> = all_classes
        .iter()
        .filter(|c| !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .chain(collection_names.iter().cloned())
        .collect();

    // Add nested types as deletable (they get destructor generation)
    let known_class_names: HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
    for func in &all_functions {
        if let Some(ref ret) = func.return_type {
            collect_nested_deletable_names(&ret.original, &known_class_names, &mut deletable_class_names);
        }
        for param in &func.params {
            collect_nested_deletable_names(&param.ty.original, &known_class_names, &mut deletable_class_names);
        }
    }

    let mut all_class_names: HashSet<String> =
        all_classes.iter().map(|c| c.name.clone()).collect();
    all_class_names.extend(collection_names.iter().cloned());
    all_class_names.extend(extra_typedef_names.iter().cloned());
    let all_enum_names = &symbol_table.all_enum_names;

    let handle_able_classes: HashSet<String> = all_classes
        .iter()
        .filter(|c| c.is_handle_type && !c.has_protected_destructor)
        .map(|c| c.name.clone())
        .collect();

    let ffi_ctx = TypeContext {
        current_module: "ffi",
        module_classes: &all_class_names,
        all_enums: all_enum_names,
        all_classes: &all_class_names,
        handle_able_classes: Some(&handle_able_classes),
        type_to_module: Some(&symbol_table.type_to_module),
        enum_rust_types: Some(&symbol_table.enum_rust_types),
        deletable_class_names: Some(&deletable_class_names),
```
</details>

Good analysis:
- **Line 2451**: Leave as-is. Protected-destructor classes themselves shouldn't get inherited methods (they're abstract bases).
- **Lines 2762 and 2924**: `deletable_class_names` - these are correct to keep because protected-destructor classes genuinely can't have `delete self_` called. This controls whether methods returning `OwnedPtr<T>` are generated.
- **Line 2948**: This is another `handle_able_classes` computation that needs the same fix.
- **Lines 4212 and 4399**: Keep as-is (destructor and CppDeletable are correctly skipped).

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2948-L2949">bindings.rs -1+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "        .filter(|c| c.is_handle_type)",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2948:po"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/codegen/bindings.rs","startLine":2948,"endLine":2948}
```
</details>

`crates/opencascade-binding-generator/src/resolver.rs` line 952 for the binding status:

<details>
<summary>Reading lines 948-960 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L948-L961">resolver.rs</a></summary>

```
    let short_name = crate::type_mapping::short_name_for_module(&class.name, &class.module);
    let rust_ffi_name = safe_short_name(&short_name);
    
    // Determine class binding status
    let class_status = if class.has_protected_destructor {
        BindingStatus::Excluded(ExclusionReason::ProtectedDestructor)
    } else {
        BindingStatus::Included
    };
    
    // Resolve constructors
    let mut constructor_ids = Vec::new();
    for (idx, ctor) in class.constructors.iter().enumerate() {
```
</details>

This status is used for the inspect output. Now that we're including protected-destructor classes, their status should be `Included` since they do get bindings (just not ctor/dtor). Let me update this:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L951-L955">resolver.rs -6+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    // Determine class binding status\n    // Protected-destructor classes are now included (methods, statics, handles)\n    // with only ctor/dtor generation skipped.\n    let class_status = BindingStatus::Included;",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "951:lp,952:vi,953:cf,954:ui,955:mx,956:jv"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":951,"endLine":954}
```
</details>

Now let me check if `ExclusionReason::ProtectedDestructor` is still used anywhere or if I should remove it:

Searched for text `ProtectedDestructor` (`**/crates/opencascade-binding-generator/**`), 1 result
- [resolver.rs:60](../crates/opencascade-binding-generator/src/resolver.rs#L60)

It's only declared now but never used. Let me remove it:

<details>
<summary>Reading lines 55-65 of <a href="../crates/opencascade-binding-generator/src/resolver.rs#L55-L66">resolver.rs</a></summary>

```
    /// Method uses an enum type that can't be bound (enum class requires integer conversion at FFI boundary)
    UsesEnum { enum_name: String },
    /// Class is abstract (has pure virtual methods)
    AbstractClass,
    /// Class has protected/private destructor
    ProtectedDestructor,
    /// Method needs explicit lifetimes (&mut self return with reference params)
    NeedsExplicitLifetimes,
    /// Method has unsupported by-value parameter (class or handle type)
    UnsupportedByValueParam { param_name: String, type_name: String },
    /// Method has const/mut return mismatch
```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/resolver.rs#L59-L60">resolver.rs -3+1</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    /// Method needs explicit lifetimes (&mut self return with reference params)",
      "filePath": "crates/opencascade-binding-generator/src/resolver.rs",
      "lineHashes": "59:zw,60:at,61:ak"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-binding-generator/src/resolver.rs","startLine":59,"endLine":59}
```
</details>

Now let me check if there are any other references to the old `ProtectedDestructor` variant:

Searched for text `ProtectedDestructor` (`**/crates/opencascade-binding-generator/**`), no results

Good, it's removed. Now let me also check the `protected_destructor_class_names()` method usage. Let me also handle the case where `protected_destructor_class_names` is used for opaque type filtering in rust.rs:

<details>
<summary>Searched for text <code>protected_destructor_class_names</code> (<code>**/crates/opencascade-binding-generator/**</code>), 7 results</summary>

- [resolver.rs:465](../crates/opencascade-binding-generator/src/resolver.rs#L465)
- [rust.rs:140](../crates/opencascade-binding-generator/src/codegen/rust.rs#L140)
- [rust.rs:170](../crates/opencascade-binding-generator/src/codegen/rust.rs#L170)
- [bindings.rs:2307](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2307)
- [bindings.rs:2357](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2357)
- [bindings.rs:2401](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2401)
</details>

Let me check how it's used in the opaque declaration generation and the inherited method logic:

<details>
<summary>Reading lines 135-185 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L135-L186">rust.rs</a></summary>

```
    symbol_table: &crate::resolver::SymbolTable,
    all_bindings: &[super::bindings::ClassBindings],
    function_bindings: &[super::bindings::FunctionBinding],
) -> (String, Vec<NestedTypeInfo>) {
    // Get all classes with protected destructors
    let protected_destructor_class_names = symbol_table.protected_destructor_class_names();

    // All enum names (needed for opaque type filtering)
    let all_enum_names = &symbol_table.all_enum_names;

    // Collect collection type names to exclude from class generation
    // Collections are generated separately with specialized wrappers
    let collection_type_names: HashSet<String> = collections.iter()
        .map(|c| c.typedef_name.clone())
        .collect();

    // Emit ffi declarations from pre-computed ClassBindings
    let class_items: String = all_bindings
        .iter()
        .filter(|b| !collection_type_names.contains(&b.cpp_name))
        .map(|b| super::bindings::emit_ffi_class(b))
        .collect();

    // Generate namespace-level free functions from pre-computed FunctionBindings
    let function_items = generate_functions_from_bindings(function_bindings);

    // Generate Handle type declarations
    let handle_decls = generate_handle_declarations(all_classes, &symbol_table.handle_able_classes);

    // Collect opaque type declarations (types referenced but not defined)
    let collected_types = collect_referenced_types(all_classes);
    let (opaque_type_decls, nested_types) = generate_opaque_declarations(
        &collected_types,
        all_classes,
        all_enum_names,
        &protected_destructor_class_names,
        &collection_type_names,
    );

    // Generate nested type destructor declarations for ffi extern block
    let nested_destructor_decls = if nested_types.is_empty() {
        String::new()
    } else {
        let mut s = String::new();
        writeln!(s).unwrap();
        writeln!(s, "    // ========================").unwrap();
        writeln!(s, "    // Nested type destructors").unwrap();
        writeln!(s, "    // ========================").unwrap();
        writeln!(s).unwrap();
        for nt in &nested_types {
            writeln!(s, "    pub fn {}_destructor(self_: *mut {});", nt.ffi_name, nt.ffi_name).unwrap();
```
</details>

Let me check what `generate_opaque_declarations` does with the protected_destructor_class_names:

Searched for text `generate_opaque_declarations` (`**/crates/opencascade-binding-generator/src/codegen/rust.rs`), 2 results
- [rust.rs:166](../crates/opencascade-binding-generator/src/codegen/rust.rs#L166)
- [rust.rs:461](../crates/opencascade-binding-generator/src/codegen/rust.rs#L461)

<details>
<summary>Reading lines 461-550 of <a href="../crates/opencascade-binding-generator/src/codegen/rust.rs#L461-L551">rust.rs</a></summary>

```
fn generate_opaque_declarations(
    collected_types: &CollectedTypes,
    classes: &[&ParsedClass],
    all_enum_names: &HashSet<String>,
    protected_destructor_classes: &HashSet<String>,
    collection_type_names: &HashSet<String>,
) -> (String, Vec<NestedTypeInfo>) {
    let defined_classes: HashSet<_> = classes.iter().map(|c| c.name.clone()).collect();
    let mut out = String::new();
    let mut emitted: HashSet<String> = HashSet::new();
    let mut nested_types: Vec<NestedTypeInfo> = Vec::new();

    for type_name in &collected_types.classes {
        if defined_classes.contains(type_name) {
            continue;
        }
        if all_enum_names.contains(type_name) {
            continue;
        }
        // Protected destructor classes still need opaque declarations when referenced
        // in method signatures; they just won't get CppDeletable.
        let has_protected_dtor = protected_destructor_classes.contains(type_name);
        if is_primitive_type(type_name) {
            continue;
        }
        // Skip collection types - they're generated separately
        if collection_type_names.contains(type_name) {
            continue;
        }
        // Nested C++ types (e.g., "Poly_CoherentTriangulation::TwoIntegers") get
        // flattened to valid Rust identifiers ("Poly_CoherentTriangulation_TwoIntegers")
        let is_nested = type_name.contains("::");
        let safe_name = if is_nested {
            type_name.replace("::", "_")
        } else {
            type_name.clone()
        };
        // Skip types with pointer/ref qualifiers leaked into the name
        // (e.g., "IMeshData_Edge *const" from typedef resolution)
        if safe_name.contains('*') || safe_name.contains('&') {
            continue;
        }
        // Avoid duplicate opaque declarations (flattened nested name might collide
        // with an existing class name or another nested type)
        if defined_classes.contains(&safe_name) || !emitted.insert(safe_name.clone()) {
            continue;
        }

        writeln!(out, "/// Referenced type from C++").unwrap();
        writeln!(out, "#[repr(C)]").unwrap();
        writeln!(out, "pub struct {} {{ _opaque: [u8; 0] }}", safe_name).unwrap();

        // Track nested types for destructor generation (skip protected destructor types)
        if is_nested && !has_protected_dtor {
            nested_types.push(NestedTypeInfo {
                cpp_name: type_name.clone(),
                ffi_name: safe_name,
            });
        }
    }

    (out, nested_types)
}

// UniquePtr impl blocks are no longer needed with extern "C" FFI

/// Emit a Rust `#[repr(i32)]` enum definition with TryFrom/From impls
fn emit_rust_enum(output: &mut String, resolved: &crate::resolver::ResolvedEnum) {
    // Doc comment
    if let Some(ref comment) = resolved.doc_comment {
        for line in comment.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(output, "///").unwrap();
            } else {
                writeln!(output, "/// {}", trimmed).unwrap();
            }
        }
    }
    writeln!(output, "/// C++ enum: `{}`", resolved.cpp_name).unwrap();

    // Collect unique variants (skip duplicated values — C++ allows alias enum values, Rust doesn't)
    let mut seen_values = std::collections::HashSet::new();
    let mut unique_variants = Vec::new();
    let mut next_value: i64 = 0;
    for variant in &resolved.variants {
        let value = variant.value.unwrap_or(next_value);
        if seen_values.insert(value) {
            unique_variants.push((variant, value));
        }
```
</details>

The `protected_destructor_classes` in  is used only for the nested type destructor check (line 514), which is correct — nested types inside protected-destructor classes shouldn't get destructors. This usage is fine to keep.

Now let me also check the `crates/opencascade-binding-generator/src/codegen/bindings.rs` lines 2307, 2357, 2401 where  is used:

<details>
<summary>Reading lines 2300-2320 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2300-L2321">bindings.rs</a></summary>

```

// ── Upcast bindings ─────────────────────────────────────────────────────────

fn compute_upcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
) -> Vec<UpcastBinding> {
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;

    all_ancestors
        .iter()
        .filter(|base| {
            !protected_destructor_classes.contains(*base)
                && symbol_table.all_class_names.contains(*base)
        })
        .map(|base_class| {
            let base_ffi_name = base_class.replace("::", "_");
            let ffi_fn_name = format!("{}_as_{}", cpp_name, base_ffi_name);
```
</details>

<details>
<summary>Reading lines 2350-2370 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2350-L2371">bindings.rs</a></summary>

```
// ── Handle upcast bindings ──────────────────────────────────────────────────

fn compute_handle_upcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
) -> Vec<HandleUpcastBinding> {
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;

    let handle_type_name = format!("Handle{}", cpp_name.replace("_", ""));

    all_ancestors
        .iter()
        .filter(|base| {
            if protected_destructor_classes.contains(*base) {
                return false;
            }
            if !handle_able_classes.contains(*base) {
```
</details>

<details>
<summary>Reading lines 2395-2420 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2395-L2421">bindings.rs</a></summary>

```

fn compute_handle_downcast_bindings(
    class: &ParsedClass,
    symbol_table: &SymbolTable,
    handle_able_classes: &HashSet<String>,
) -> Vec<HandleDowncastBinding> {
    let protected_destructor_classes = symbol_table.protected_destructor_class_names();
    let all_descendants = symbol_table.get_all_descendants_by_name(&class.name);
    let cpp_name = class.name.replace("::", "_");
    let cpp_name = &cpp_name;

    let handle_type_name = format!("Handle{}", cpp_name.replace("_", ""));

    all_descendants
        .iter()
        .filter(|desc| {
            if protected_destructor_classes.contains(*desc) {
                return false;
            }
            if !handle_able_classes.contains(*desc) {
                return false;
            }
            if let Some(desc_class) = symbol_table.class_by_name(desc) {
                // Only generate downcasts to concrete (non-abstract) descendants
                // that are handle types
                desc_class.is_handle_type && !desc_class.is_abstract
```
</details>

These upcast/downcast functions filter out protected-destructor classes from the targets. Now that we're generating bindings for protected-destructor classes too, we should allow upcasting/downcasting to them. Let me update these:

<details>
<summary>Edited: <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2303-L2316">bindings.rs -15+13</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2350-L2365">-19+15</a>, <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2390-L2411">-25+21</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "fn compute_upcast_bindings(\n    class: &ParsedClass,\n    symbol_table: &SymbolTable,\n) -> Vec<UpcastBinding> {\n    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);\n    let cpp_name = class.name.replace(\"::\", \"_\");\n    let cpp_name = &cpp_name;\n\n    all_ancestors\n        .iter()\n        .filter(|base| {\n            symbol_table.all_class_names.contains(*base)\n        })",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2303:ig,2304:ei,2305:ca,2306:np,2307:jc,2308:gk,2309:oe,2310:ka,2311:dd,2312:rd,2313:cb,2314:re,2315:qh,2316:gp,2317:lq"
    },
    {
      "content": "fn compute_handle_upcast_bindings(\n    class: &ParsedClass,\n    symbol_table: &SymbolTable,\n    handle_able_classes: &HashSet<String>,\n) -> Vec<HandleUpcastBinding> {\n    let all_ancestors = symbol_table.get_all_ancestors_by_name(&class.name);\n    let cpp_name = class.name.replace(\"::\", \"_\");\n    let cpp_name = &cpp_name;\n\n    let handle_type_name = format!(\"Handle{}\", cpp_name.replace(\"_\", \"\"));\n\n    all_ancestors\n        .iter()\n        .filter(|base| {\n            if !handle_able_classes.contains(*base) {",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2352:pb,2353:ei,2354:ca,2355:kf,2356:zc,2357:jc,2358:gk,2359:oe,2360:ka,2361:dd,2362:hy,2363:dd,2364:rd,2365:cb,2366:re,2367:pi,2368:ta,2369:uo,2370:nj"
    },
    {
      "content": "fn compute_handle_downcast_bindings(\n    class: &ParsedClass,\n    symbol_table: &SymbolTable,\n    handle_able_classes: &HashSet<String>,\n) -> Vec<HandleDowncastBinding> {\n    let all_descendants = symbol_table.get_all_descendants_by_name(&class.name);\n    let cpp_name = class.name.replace(\"::\", \"_\");\n    let cpp_name = &cpp_name;\n\n    let handle_type_name = format!(\"Handle{}\", cpp_name.replace(\"_\", \"\"));\n\n    all_descendants\n        .iter()\n        .filter(|desc| {\n            if !handle_able_classes.contains(*desc) {\n                return false;\n            }\n            if let Some(desc_class) = symbol_table.class_by_name(desc) {\n                // Only generate downcasts to concrete (non-abstract) descendants\n                // that are handle types\n                desc_class.is_handle_type && !desc_class.is_abstract",
      "filePath": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "lineHashes": "2396:yh,2397:ei,2398:ca,2399:kf,2400:es,2401:jc,2402:kd,2403:oe,2404:ka,2405:dd,2406:hy,2407:dd,2408:rx,2409:cb,2410:fn,2411:rd,2412:ta,2413:uo,2414:bn,2415:ta,2416:uo,2417:tr,2418:zf,2419:nm,2420:rz"
    }
  ]
}
```
**Output:**
```
{
  "applied": 3,
  "failed": 0,
  "files": [
    {
      "file": "crates/opencascade-binding-generator/src/codegen/bindings.rs",
      "startLine": 2303,
      "endLine": 2410
    }
  ]
}
```
</details>

Now let's also update the bindings.toml to remove TDF_LabelNode from the exclude list (since we now properly handle the class) and update the comment:

Wait, actually TDF_LabelNode should stay excluded because it's truly internal with no public API exposure. The issue there isn't just the protected destructor — it has custom allocator hiding standard new/delete. Let me keep the exclude but update the comment.

Actually, looking at it more carefully: now that we handle protected destructors, TDF_LabelNode should also work (it would just have methods but no ctor/dtor). However, since it's internal-only and useless, there's no point in including it. Let me leave the exclude as-is.

Now let me try to build the binding generator to check for compile errors:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build -p opencascade-binding-generator 2>&1
```
```
   Compiling opencascade-binding-generator v0.1.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-binding-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.56s
real  2.6s
```

Compiles cleanly. Let me also check for warnings:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && cargo build -p opencascade-binding-generator 2>&1 | grep -i "warn"
```
```
(base) whitelily:opencascade-rs sethml$
```
**Exit code:** 1

No warnings. Now let me regenerate bindings and build:

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- ✅ Check for other excluded non-constructible classes
- ✅ Implement non-constructible class support
- 🔄 Test and validate changes
- ⬜ Commit changes

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/regenerate-bindings.sh 2>&1
```

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -20 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01Q7yjmgbQk3KKDVHGSGR4kA__vscode-1771312782450/content.txt
```
<details>
<summary>Output (20 lines)</summary>

```
ap203.rs (extra types only, 4 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/xcaf_
view.rs (extra types only, 1 types)                                               Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/iges_
basic.rs (extra types only, 1 types)                                              Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/prox_
pnt.rs (extra types only, 1 types)                                                Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_s_
lib.rs (function-only module, 76 functions)                                       Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/b_rep
_bnd_lib.rs (function-only module, 4 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/el_c_
lib.rs (function-only module, 120 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/geom_
proj_lib.rs (function-only module, 8 functions)                                   Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/rw_ob
j_tools.rs (function-only module, 1 functions)                                    Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/n_col
lection_primes.rs (function-only module, 1 functions)                             Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/preci
sion.rs (function-only module, 18 functions)                                      Wrote: /Users/sethml/src/opencascade-rs/crates/opencascade-sys/generated/lib.r
s                                                                               
Formatting generated Rust code with rustfmt...
  Formatting complete.

Code generation complete!
  213 modules generated

Bindings generated in /Users/sethml/src/opencascade-rs/crates/opencascade-sys/ge
nerated                                                                         real  15.4s
```
</details>

Regeneration succeeded. Now let me build the full project:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo build 2>&1
```

Full build succeeded. Now let me run the tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time scripts/run-binding-generator-tests.sh 2>&1
```

All 16 tests pass. Now let me run the workspace tests:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && time cargo test --workspace --exclude opencascade-binding-generator 2>&1
```
<details>
<summary>Output (345 lines)</summary>

```
   Compiling examples v0.2.0 (/Users/sethml/src/opencascade-rs/examples)
   Compiling opencascade-sys v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade-sys)
   Compiling opencascade v0.2.0 (/Users/sethml/src/opencascade-rs/crates/opencascade)
   Compiling viewer v0.2.0 (/Users/sethml/src/opencascade-rs/crates/viewer)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.50s
     Running unittests src/lib.rs (target/debug/deps/examples-638415a022e1f97e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/write_model.rs (target/debug/deps/write_model-4fb5f107f01498a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/golden.rs (target/debug/deps/golden-a67f7fbf81fc69ec)

running 23 tests
test high_level_bottle ... ignored, non-deterministic STEP output

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 2                      ******

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 0                      ******
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/T/.tmpgnB7tM/airfoil.step(222 ents)  Write  Done
** WorkSession : Sending all data
 Step File Name : /var/folders/r4/_7ft0br93fxgk7jjkwpf5xyc0000gn/T/.tmpvXS9Lf/bounding-box.step(625 ents)  Write  Done
test airfoil ... ok
test bounding_box ... ok

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 0                      ******
** WorkSession : Sending all data

*******************************************************************
******        Statistics on Transfer (Write)                 ******

*******************************************************************
******        Transfer Mode = 0  I.E.  As Is       ******
******        Transferring Shape, ShapeType = 2                      ******

*******************************************************************
******        Statistics on T
... (truncated, 16951 chars total)
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && tail -5 /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01C1Kg5g3tFbyZpatu9rys1w__vscode-1771312782454/content.txt
```
<details>
<summary>Output (5 lines)</summary>

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
shed in 0.00s                                                                   
real  942.4s
```
</details>

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -E "test result:|FAILED" /Users/sethml/Library/Application\ Support/Code/User/workspaceStorage/140fb29cc18995a7b3c75038586f8e11/GitHub.copilot-chat/chat-session-resources/e17d9a0b-55f8-4fa5-bbbe-fb84dce38361/toolu_01C1Kg5g3tFbyZpatu9rys1w__vscode-1771312782454/content.txt
```
<details>
<summary>Output (17 lines)</summary>

```
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 22 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; fin
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fini
```
</details>

All tests pass (0 failures anywhere). Let me now verify what was actually added by looking at the diff:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git diff --stat HEAD
```
<details>
<summary>Output (31 lines)</summary>

```
 .../src/codegen/bindings.rs                        |   25 +-
 .../src/codegen/cpp.rs                             |    7 +-
 .../src/codegen/rust.rs                            |   19 +-
 crates/opencascade-binding-generator/src/main.rs   |   22 +-
 .../opencascade-binding-generator/src/resolver.rs  |   18 +-
 .../src/type_mapping.rs                            |    4 +-
 crates/opencascade-sys/bindings.toml               |    2 -
 crates/opencascade-sys/generated/b_rep_algo_api.rs |  137 ++
 crates/opencascade-sys/generated/b_rep_class.rs    |   16 +
 crates/opencascade-sys/generated/b_rep_feat.rs     |   22 +
 crates/opencascade-sys/generated/b_rep_mesh.rs     |  101 ++
 crates/opencascade-sys/generated/bop_algo.rs       |  298 +++++
 crates/opencascade-sys/generated/bopds.rs          |  206 +++
 crates/opencascade-sys/generated/cdf.rs            |  165 +++
 crates/opencascade-sys/generated/cdm.rs            |  636 +++++++++
 crates/opencascade-sys/generated/ffi.rs            | 1362 +++++++++++++++++++-
 crates/opencascade-sys/generated/geom2d_hatch.rs   |   16 +
 crates/opencascade-sys/generated/geom2d_int.rs     |   66 +
 crates/opencascade-sys/generated/hatch_gen.rs      |  160 +++
 crates/opencascade-sys/generated/hlrb_rep.rs       |  116 ++
 crates/opencascade-sys/generated/int_curve.rs      |   34 +
 .../opencascade-sys/generated/int_curve_surface.rs |   99 ++
 crates/opencascade-sys/generated/int_res2d.rs      |   72 ++
 crates/opencascade-sys/generated/intf.rs           |  103 ++
 crates/opencascade-sys/generated/n_collection.rs   |  104 +-
 crates/opencascade-sys/generated/osd.rs            |  126 ++
 crates/opencascade-sys/generated/t_doc_std.rs      |  191 +++
 crates/opencascade-sys/generated/t_obj.rs          |  451 +++++++
 crates/opencascade-sys/generated/wrappers.cpp      |  950 ++++++++++++++
 crates/opencascade-sys/generated/xcaf_app.rs       |   75 ++
 30 files changed, 5526 insertions(+), 77 deletions(-)
```
</details>

Excellent! This is a substantial addition — 5,526 new lines of bindings. Let me verify some key classes are now bound:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep -c "pub fn" crates/opencascade-sys/generated/bop_algo.rs crates/opencascade-sys/generated/b_rep_algo_api.rs crates/opencascade-sys/generated/int_res2d.rs crates/opencascade-sys/generated/int_curve_surface.rs crates/opencascade-sys/generated/intf.rs crates/opencascade-sys/generated/osd.rs crates/opencascade-sys/generated/t_obj.rs crates/opencascade-sys/generated/b_rep_mesh.rs
```
<details>
<summary>Output (8 lines)</summary>

```
crates/opencascade-sys/generated/bop_algo.rs:1318
crates/opencascade-sys/generated/b_rep_algo_api.rs:354
crates/opencascade-sys/generated/int_res2d.rs:54
crates/opencascade-sys/generated/int_curve_surface.rs:192
crates/opencascade-sys/generated/intf.rs:90
crates/opencascade-sys/generated/osd.rs:553
crates/opencascade-sys/generated/t_obj.rs:576
crates/opencascade-sys/generated/b_rep_mesh.rs:488
```
</details>

Let me check a specific example — the key BRepAlgoAPI_Algo methods that should now appear:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "BRepAlgoAPI_Algo" crates/opencascade-sys/generated/b_rep_algo_api.rs | grep -v "//"
```
<details>
<summary>Output (22 lines)</summary>

```
pub use crate::ffi::BRepAlgoAPI_Algo as Algo;
        unsafe { &*(crate::ffi::BRepAlgoAPI_Algo_shape(self as *mut Self)) }
        unsafe { &*(crate::ffi::BRepAlgoAPI_Algo_as_BRepBuilderAPI_MakeShape(self as *const Self)) }
            &mut *(crate::ffi::BRepAlgoAPI_Algo_as_BRepBuilderAPI_MakeShape_mut(self as *mut Self))
        unsafe { &*(crate::ffi::BRepAlgoAPI_Algo_as_BRepBuilderAPI_Command(self as *const Self)) }
            &mut *(crate::ffi::BRepAlgoAPI_Algo_as_BRepBuilderAPI_Command_mut(self as *mut Self))
            &*(crate::ffi::BRepAlgoAPI_BooleanOperation_as_BRepAlgoAPI_Algo(self as *const Self))
            &mut *(crate::ffi::BRepAlgoAPI_BooleanOperation_as_BRepAlgoAPI_Algo_mut(
        unsafe { &*(crate::ffi::BRepAlgoAPI_BuilderAlgo_as_BRepAlgoAPI_Algo(self as *const Self)) }
            &mut *(crate::ffi::BRepAlgoAPI_BuilderAlgo_as_BRepAlgoAPI_Algo_mut(self as *mut Self))
        unsafe { &*(crate::ffi::BRepAlgoAPI_Common_as_BRepAlgoAPI_Algo(self as *const Self)) }
        unsafe { &mut *(crate::ffi::BRepAlgoAPI_Common_as_BRepAlgoAPI_Algo_mut(self as *mut Self)) }
        unsafe { &*(crate::ffi::BRepAlgoAPI_Cut_as_BRepAlgoAPI_Algo(self as *const Self)) }
        unsafe { &mut *(crate::ffi::BRepAlgoAPI_Cut_as_BRepAlgoAPI_Algo_mut(self as *mut Self)) }
        unsafe { &*(crate::ffi::BRepAlgoAPI_Defeaturing_as_BRepAlgoAPI_Algo(self as *const Self)) }
            &mut *(crate::ffi::BRepAlgoAPI_Defeaturing_as_BRepAlgoAPI_Algo_mut(self as *mut Self))
        unsafe { &*(crate::ffi::BRepAlgoAPI_Fuse_as_BRepAlgoAPI_Algo(self as *const Self)) }
        unsafe { &mut *(crate::ffi::BRepAlgoAPI_Fuse_as_BRepAlgoAPI_Algo_mut(self as *mut Self)) }
        unsafe { &*(crate::ffi::BRepAlgoAPI_Section_as_BRepAlgoAPI_Algo(self as *const Self)) }
            &mut *(crate::ffi::BRepAlgoAPI_Section_as_BRepAlgoAPI_Algo_mut(self as *mut Self))
        unsafe { &*(crate::ffi::BRepAlgoAPI_Splitter_as_BRepAlgoAPI_Algo(self as *const Self)) }
            &mut *(crate::ffi::BRepAlgoAPI_Splitter_as_BRepAlgoAPI_Algo_mut(self as *mut Self))
```
</details>

`BRepAlgoAPI_Algo` is now fully bound with `Shape()` method and all subclasses can upcast to it. Let me verify the Handle generation for `BRepMesh_IncrementalMesh`:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "IncrementalMesh" crates/opencascade-sys/generated/b_rep_mesh.rs | grep -v "//" | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
pub use crate::ffi::BRepMesh_IncrementalMesh as IncrementalMesh;
unsafe impl crate::CppDeletable for IncrementalMesh {
        crate::ffi::BRepMesh_IncrementalMesh_destructor(ptr);
impl IncrementalMesh {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::BRepMesh_IncrementalMesh_ctor()) }
                crate::ffi::BRepMesh_IncrementalMesh_ctor_shape_real_bool_real_bool(
                crate::ffi::BRepMesh_IncrementalMesh_ctor_shape_parameters_progressrange(
        unsafe { crate::ffi::BRepMesh_IncrementalMesh_perform(self as *mut Self, theRange) }
        unsafe { &*(crate::ffi::BRepMesh_IncrementalMesh_parameters(self as *const Self)) }
        unsafe { &mut *(crate::ffi::BRepMesh_IncrementalMesh_change_parameters(self as *mut Self)) }
        unsafe { crate::ffi::BRepMesh_IncrementalMesh_is_modified(self as *const Self) }
        unsafe { crate::ffi::BRepMesh_IncrementalMesh_get_status_flags(self as *const Self) }
        unsafe { &*(crate::ffi::BRepMesh_IncrementalMesh_dynamic_type(self as *const Self)) }
        unsafe { crate::ffi::BRepMesh_IncrementalMesh_is_parallel_default() }
        unsafe { crate::ffi::BRepMesh_IncrementalMesh_set_parallel_default(isInParallel) }
            std::ffi::CStr::from_ptr(crate::ffi::BRepMesh_IncrementalMesh_get_type_name())
        unsafe { &*(crate::ffi::BRepMesh_IncrementalMesh_get_type_descriptor()) }
            &*(crate::ffi::BRepMesh_IncrementalMesh_as_BRepMesh_DiscretRoot(self as *const Self))
            &mut *(crate::ffi::BRepMesh_IncrementalMesh_as_BRepMesh_DiscretRoot_mut(
            crate::ffi::BRepMesh_IncrementalMesh_inherited_SetShape(self as *mut Self, theShape)
```
</details>

`BRepMesh_IncrementalMesh` has full bindings including constructors, `Perform()`, `GetStatusFlags()`, `CppDeletable`, etc. 

Wait - I notice BRepMesh_IncrementalMesh has a full `CppDeletable` impl and constructors. It seems it actually has a public destructor (the Python script false-positive). This confirms it was being unnecessarily excluded before. Let me also quickly check the Handle types for protected-destructor classes:

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && grep "HandleBRepMeshDiscretFactory\|HandleBRepMeshIncrementalMesh\|HandleTObjModel\|HandleBOPAlgoAlgo" crates/opencascade-sys/generated/ffi.rs | head -20
```
<details>
<summary>Output (20 lines)</summary>

```
pub struct HandleTObjModel {
    pub fn TObj_Assistant_find_model(theName: *const std::ffi::c_char) -> *mut HandleTObjModel;
    pub fn TObj_Assistant_bind_model(theModel: *const HandleTObjModel);
    pub fn TObj_Assistant_set_current_model(theModel: *const HandleTObjModel);
    pub fn TObj_Assistant_get_current_model() -> *mut HandleTObjModel;
        theModel: *const HandleTObjModel,
    pub fn TObj_CheckModel_get_model(self_: *const TObj_CheckModel) -> *const HandleTObjModel;
    ) -> *mut HandleTObjModel;
        theModel: *const HandleTObjModel,
    pub fn TObj_Model_new_empty(self_: *mut TObj_Model) -> *mut HandleTObjModel;
        theTarget: *const HandleTObjModel,
    pub fn TObj_Model_get_document_model(theLabel: *const TDF_Label) -> *mut HandleTObjModel;
    pub fn HandleTObjModel_destructor(self_: *mut HandleTObjModel);
    pub fn HandleTObjModel_get(handle: *const HandleTObjModel) -> *const TObj_Model;
    pub fn HandleTObjModel_get_mut(handle: *mut HandleTObjModel) -> *mut TObj_Model;
        theModel: *const HandleTObjModel,
    pub fn TObj_Object_get_model(self_: *const TObj_Object) -> *mut HandleTObjModel;
    pub fn TObj_Partition_inherited_GetModel(self_: *const TObj_Partition) -> *mut HandleTObjModel;
    pub fn TObj_TModel_set(self_: *mut TObj_TModel, theModel: *const HandleTObjModel);
    pub fn TObj_TModel_model(self_: *const TObj_TModel) -> *mut HandleTObjModel;
```
</details>

`HandleTObjModel` is now generated with destructor, get, and get_mut — even though `TObj_Model` has a protected destructor. The Handle manages the lifetime safely.

Let me now also verify that the OSD_FileNode base class methods are properly bound:

*2026-02-18 16:27 · 4218s · Words: 54,471 in · 6,534 out · 95 rounds*

---

<a id="user-2"></a>

## User (2)

> @agent Try Again

*2026-02-18 17:15*

### Assistant — Claude Opus 4.6

`HandleTObjModel` is now generated - a Handle type for the protected-destructor class `TObj_Model`. The Handle manages its lifetime safely.

Now let me update the bindings.toml comment to reflect the new behavior:

<details>
<summary>Reading lines 230-250 of <a href="../crates/opencascade-sys/bindings.toml#L230-L251">bindings.toml</a></summary>

```

    # No suitable operator delete for Poly_CoherentTriPtr.
    "Poly_CoherentTriPtr.hxx",

    # Deleted copy constructor: TopoDS_AlertAttribute can't be copy-constructed.
    "TopoDS_AlertAttribute.hxx",

    # Third-party dependency (rapidjson): these headers #include rapidjson which
    # is not bundled with our OCCT build.
    "RWGltf_GltfOStreamWriter.hxx",
    "RWGltf_GltfJsonParser.hxx",
    "RWGltf_CafWriter.hxx",
    "RWGltf_GltfAccessor.hxx",

    # BVH template classes: abstract base or complex template type params
    # that the generator can't resolve correctly.
    "BVH_Properties.hxx",

    # Protected operator delete (TDF_LabelNode).
    "TDF_LabelNode.hxx",

```
</details>

<details>
<summary>Edited: <a href="../crates/opencascade-sys/bindings.toml#L248-L252">bindings.toml -2+4</a></summary>

**Input:**
```
{
  "edits": [
    {
      "content": "    # Internal-only class: all constructors/destructors are private, accessible only\n    # to friend classes TDF_Data and TDF_Label. No public API returns TDF_LabelNode*;\n    # it's entirely hidden behind the TDF_Label value-type facade.\n    \"TDF_LabelNode.hxx\",",
      "filePath": "crates/opencascade-sys/bindings.toml",
      "lineHashes": "248:lw,249:qb"
    }
  ]
}
```
**Output:**
```
{"status":"ok","applied":1,"file":"crates/opencascade-sys/bindings.toml","startLine":248,"endLine":251}
```
</details>

Now let me also update the "Problem Classes" section in bindings.toml to document this improvement:

<details>
<summary>Reading lines 265-280 of <a href="../crates/opencascade-sys/bindings.toml#L265-L281">bindings.toml</a></summary>

```
# When clang can't fully instantiate certain template types, it misresolves them to
# Standard_Integer (int). This corruption affects both canonical AND display names,
# so the parser's heuristic guards cannot detect it. These methods are excluded
# individually so the rest of each class remains accessible.
# Format: "ClassName::MethodName" for methods, "ClassName::ClassName" for constructors.
exclude_methods = [
    # math_Matrix: Row() and Col() return math_VectorBase<> (typedef math_Vector)
    # which clang misresolves to Standard_Integer in batch parsing mode.
    "math_Matrix::Row",
    "math_Matrix::Col",

    # TObj_TIntSparseArray: GetIterator() returns NCollection_SparseArray<int>::ConstIterator
    # which clang misresolves to Standard_Integer in batch parsing mode.
    "TObj_TIntSparseArray::GetIterator",

    # XSAlgo_ShapeProcessor: constructor takes NCollection_DataMap param (class-scoped
```
</details>

<details>
<summary>Reading lines 295-330 of <a href="../crates/opencascade-sys/bindings.toml#L295-L331">bindings.toml</a></summary>

```
resolve_deps = true

# =============================================================================
# Problem Classes Preventing Header Inclusion
# =============================================================================
#
# The following categories of problems prevent certain OCCT headers from being
# included in the binding generation. Each category describes the root cause,
# affected headers, and potential solutions.
#
# 1. NCollection Template Parameter Misresolution (SOLVED)
#    ─────────────────────────────────────────────────────────────────
#    Root cause: When clang processes NCollection template specializations
#    (e.g., NCollection_DataMap<TDF_Label, TDF_Label>) that it cannot fully
#    instantiate, it falls back to canonical type "int" (Standard_Integer).
#    This causes constructor and method parameters/returns to have wrong types.
#
#    Solution implemented: Three-layer guard in parser.rs:
#    a) AST-based typedef map — collect_ncollection_typedefs() pre-scans all
#       typedef declarations, building a map from NCollection template spelling
#       to typedef name. lookup_typedef() then resolves template display names
#       back to their original typedef (e.g., NCollection_DataMap<K,V> →
#       TopTools_DataMapOfShapeShape). normalize_template_spelling() handles
#       Standard_Integer→int, Standard_Real→double type alias mismatches.
#    b) spelling_looks_like_class guard — when a non-template, non-namespace
#       display name looks like an OCCT class but canonical type is "int",
#       checks the underlying typedef declaration to distinguish real primitive
#       typedefs from misresolved NCollection types.
#    c) Reference/pointer canonical guard — when the outer ref/ptr type's
#       display name identifies an OCCT class/template/namespace but canonical
#       resolves to "int", constructs the Class type directly instead of
#       recursing into the pointee (which would lose the typedef info).
#
#    Additionally, is_constructor_bindable() in codegen/bindings.rs now checks
#    type_uses_unknown_type() to filter constructors with unresolvable types.
#
```
</details>

<details>
<summary>Reading lines 380-420 of <a href="../crates/opencascade-sys/bindings.toml#L380-L421">bindings.toml</a></summary>

```
#    ─────────────────────
#    Some classes reference external libraries (e.g., RapidJSON, RWGltf DRACO
#    compression) that may not be available in the build environment.
#
#    Affected: RWGltf_GltfMaterialMap, RWGltf_GltfOStreamWriter,
#    RWGltf_GltfSceneNodeMap, RWGltf_CafReader, RWMesh_FaceIterator,
#    RWObj_CafReader, StdStorage_Root
#
# 5. Name Collisions and Method Conflicts (PARTIALLY SOLVED)
#    ─────────────────────────────────────────────────────────
#    Some headers produce duplicate method names or conflict with Rust
#    reserved words in ways the generator can't resolve.
#
#    Rust prelude collision (SOLVED): BRepCheck_Result's short name "Result"
#    collided with Rust's prelude Result<T,E>. Fixed by applying safe_short_name()
#    consistently in codegen — reserved names get trailing underscore (Result_).
#    All 8 BRepCheck headers now unblocked.
#
#    Remaining: TObj_HiddenPartition (duplicate new_label method from static
#    constructor and inherited method), TDataStd_NamedData (PascalCase vs
#    camelCase methods colliding in snake_case), IntPatch_WLine/
#    IntPatch_ImpImpIntersection (name conflicts or unmapped operator types).
#
# 6. Modules Not Included (Intentionally Excluded)
#    ─────────────────────────────────────────────
#    The following module categories are not included because they are either
#    platform-specific, visualization-only, or not relevant to CAD modeling:
#
#    - Platform-specific GUI: Cocoa, WNT, Wasm, Xw, D3DHost, OpenGl
#    - Visualization/rendering: AIS, V3d, Graphic3d, Prs3d, PrsMgr, Select3D,
#      SelectMgr, StdPrs, StdSelect, MeshVS, XSDRAW*
#    - Font/image handling: Font, Image
#    - Persistence/Storage: FSD, Storage, StdStorage, ShapeSchema, PCDM,
#      CDM, CDF, BinDrivers, XmlDrivers
#    - Draw test harness: Draw, DBRep, BRepTest, *Draw*
#    - VRML: Vrml*
#    - LDOM/XML core: LDOM, XmlObjMgt
#    - Other unused: Media, TKernel-internal headers

```
</details>

Let me look for the section about protected destructors:

Searched for regex `protected.*destruct|non-instantiable` (`**/crates/opencascade-sys/bindings.toml`), no results

There isn't a section about protected destructors in the problem classes documentation. That's fine since it's now handled. Let me also quickly check if we have any dangling  references left in the codebase that I should verify are correct:

<details>
<summary>Searched for text <code>has_protected_destructor</code> (<code>**/crates/opencascade-binding-generator/**</code>), 20 results</summary>

- [model.rs:132](../crates/opencascade-binding-generator/src/model.rs#L132)
- [parser.rs:406](../crates/opencascade-binding-generator/src/parser.rs#L406)
- [parser.rs:413](../crates/opencascade-binding-generator/src/parser.rs#L413)
- [parser.rs:587](../crates/opencascade-binding-generator/src/parser.rs#L587)
- [resolver.rs:122](../crates/opencascade-binding-generator/src/resolver.rs#L122)
- [resolver.rs:468](../crates/opencascade-binding-generator/src/resolver.rs#L468)
- [resolver.rs:1015](../crates/opencascade-binding-generator/src/resolver.rs#L1015)
- [main.rs:465](../crates/opencascade-binding-generator/src/main.rs#L465)
- [bindings.rs:40](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L40)
- [bindings.rs:1597](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1597)
- [bindings.rs:1796](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1796)
- [bindings.rs:1805](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1805)
- [bindings.rs:1873](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L1873)
- [bindings.rs:2441](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2441)
- [bindings.rs:2752](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2752)
- [bindings.rs:2914](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L2914)
- [bindings.rs:4202](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4202)
- [bindings.rs:4389](../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4389)
</details>

Let me verify the one at line 4840:

<details>
<summary>Reading lines 4835-4850 of <a href="../crates/opencascade-binding-generator/src/codegen/bindings.rs#L4835-L4851">bindings.rs</a></summary>

```

    // Section header
    writeln!(out, "    // ======================== {} ========================", cn).unwrap();

    // ── Destructor ──────────────────────────────────────────────────────
    if !bindings.has_protected_destructor {
        writeln!(out, "    pub fn {}_destructor(self_: *mut {});", cn, cn).unwrap();
    }

    // ── Constructors (skip convenience wrappers — they are Rust-only) ──
    for ctor in bindings.constructors.iter().filter(|c| c.convenience_of.is_none()) {
        let source = format_source_attribution(
            &bindings.source_header,
            ctor.source_line,
            &format!("{}::{}()", cn, cn),
        );
```
</details>

Good - this is the FFI destructor declaration. It's correctly gated — we don't want to declare `X_destructor` for classes with protected destructors.

Let me also check that the remaining references are all fine:

- **model.rs:132** — field declaration, fine
- **parser.rs:406,413,587** — detection and storage, fine
- **resolver.rs:122,468,1015** — field declaration,  helper, and propagation, fine
- **main.rs:465** — debug output for inspect, fine
- **bindings.rs:40** — field in ClassBindings, fine
- **bindings.rs:1597** — constructor skip, correct
- **bindings.rs:1796** — to_owned skip, correct
- **bindings.rs:1805** — to_handle skip, correct
- **bindings.rs:1873** — field propagation, fine
- **bindings.rs:2441** — inherited methods skip for the base class itself, correct
- **bindings.rs:2752,2914** — deletable_class_names filter, correct
- **bindings.rs:4202** — C++ destructor skip, correct
- **bindings.rs:4389** — CppDeletable skip, correct
- **bindings.rs:4840** — FFI destructor decl skip, correct

All the remaining uses of  are correct — they gate things that genuinely can't work for protected-destructor classes (constructors, destructors, CppDeletable, to_owned, to_handle).

Now let me commit:

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- ✅ Check for other excluded non-constructible classes
- ✅ Implement non-constructible class support
- ✅ Test and validate changes
- 🔄 Commit changes

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
rm /Users/sethml/src/opencascade-rs/tmp/commit-msg.txt /Users/sethml/src/opencascade-rs/tmp/find_protected_dtors.py /Users/sethml/src/opencascade-rs/tmp/find_protected_dtors2.py 2>/dev/null; true
```

Created [commit-msg.txt](../tmp/commit-msg.txt)

**Terminal:**
```sh
cd /Users/sethml/src/opencascade-rs && git add -A && git commit -F tmp/commit-msg.txt
```
```
[main 4733816] Generate bindings for classes with protected/private destructors
 30 files changed, 5529 insertions(+), 78 deletions(-)
```

**Todo list:**
- ✅ Investigate TDF_LabelNode usage in OCCT
- ✅ Check for other excluded non-constructible classes
- ✅ Implement non-constructible class support
- ✅ Test and validate changes
- ✅ Commit changes

Done. Here's a summary:

## Investigation

**TDF_LabelNode** is an internal-only class — all constructors/destructors are private, accessible only to friend classes `TDF_Data` and `TDF_Label`. No public API returns `TDF_LabelNode*`; everything is hidden behind the `TDF_Label` value-type facade. `TDF_Label` already exposes all the same information (`Tag()`, `Depth()`, `Father()`, `IsRoot()`, etc.). Binding TDF_LabelNode would be useless.

However, the investigation revealed a bigger opportunity: **25 classes** with protected/private destructors were being completely blanket-skipped by the generator, including important ones like:

- **BRepAlgoAPI_Algo** — base class for all boolean operations (`Shape()`, `HasErrors()`, etc.)
- **IntCurveSurface_Intersection**, **IntRes2d_Intersection** — intersection result accessors
- **BRepMesh_IncrementalMesh** — key tessellation class (actually has a public dtor, was false-positive)
- **TObj_Model** — Handle-managed model base class
- **OSD_FileNode** — file system abstraction base

## What changed

 was changed from a blanket kill-switch to a targeted guard that only skips what genuinely requires constructability/destructability (ctors, dtors, CppDeletable, to_owned, to_handle). Methods, statics, upcasts, Handle generation, and module re-exports now work for all classes.

**Result: +5,529 lines of new bindings** across 30 files, all building and passing tests.

*2026-02-18 17:16 · 89s · Words: 756 in · 952 out · 109,479 ctx · 15 rounds*

---
