// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "event_dispatcher.h"
#include "simple_example_interactor_export.h"

#include <QAction>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Tools::UndoRedo;

namespace Simple::Interactor::UndoRedo
{

class SIMPLE_EXAMPLE_INTERACTOR_EXPORT UndoRedoInteractor : public QObject
{
    Q_OBJECT
  public:
    explicit UndoRedoInteractor(ThreadedUndoRedoSystem *undo_redo_system,
                                QSharedPointer<EventDispatcher> eventDispatcher);

    static UndoRedoInteractor *instance();

    Q_INVOKABLE bool canUndo() const;

    Q_INVOKABLE bool canRedo() const;

    Q_INVOKABLE void setUndoLimit(int limit);

    Q_INVOKABLE int undoLimit() const;

    Q_INVOKABLE QString undoText() const;

    Q_INVOKABLE QString redoText() const;

    Q_INVOKABLE QStringList undoRedoTextList() const;

    Q_INVOKABLE int currentIndex() const;

    Q_INVOKABLE QUuid activeStackId() const;

    Q_INVOKABLE QStringList queuedCommandTextListByScope(const QString &scopeFlagString) const;

    Q_INVOKABLE bool isRunning() const;

    Q_INVOKABLE int numberOfCommands() const;

    QAction *createUndoAction(QObject *parent, const QString &prefix = QString()) const;
    QAction *createRedoAction(QObject *parent, const QString &prefix = QString()) const;

  public Q_SLOTS:

    void undo();
    void redo();
    void clear();
    void setCurrentIndex(int index);
    void setActiveStack(const QUuid &stackId = QUuid());

  private:
    static QPointer<UndoRedoInteractor> s_instance;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    UndoRedoInteractor() = delete;
    UndoRedoInteractor(const UndoRedoInteractor &) = delete;
    UndoRedoInteractor &operator=(const UndoRedoInteractor &) = delete;
};

} // namespace Simple::Interactor::UndoRedo