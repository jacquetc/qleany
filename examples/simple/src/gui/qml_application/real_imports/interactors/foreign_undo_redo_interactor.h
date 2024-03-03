// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "undo_redo/undo_redo_interactor.h"
#include <QQmlEngine>

using namespace Simple::Interactor::UndoRedo;

struct ForeignUndoRedoInteractor
{
    Q_GADGET
    QML_FOREIGN(Simple::Interactor::UndoRedo::UndoRedoInteractor)
    QML_SINGLETON
    QML_NAMED_ELEMENT(UndoRedoInteractor)

  public:
    // Initialize this singleton instance with the given engine.

    inline static UndoRedoInteractor *s_singletonInstance = nullptr;

    static UndoRedoInteractor *create(QQmlEngine *, QJSEngine *engine)
    {
        s_singletonInstance = UndoRedoInteractor::instance();

        // The instance has to exist before it is used. We cannot replace it.
        Q_ASSERT(s_singletonInstance);

        // The engine has to have the same thread affinity as the singleton.
        Q_ASSERT(engine->thread() == s_singletonInstance->thread());

        // There can only be one engine accessing the singleton.
        if (s_engine)
            Q_ASSERT(engine == s_engine);
        else
            s_engine = engine;

        // Explicitly specify C++ ownership so that the engine doesn't delete
        // the instance.
        QJSEngine::setObjectOwnership(s_singletonInstance, QJSEngine::CppOwnership);

        return s_singletonInstance;
    }

  private:
    inline static QJSEngine *s_engine = nullptr;
};