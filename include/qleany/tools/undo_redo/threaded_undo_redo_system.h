// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#pragma once

#include "qleany/qleany_export.h"
#include "undo_redo_system.h"
#ifdef QLEANY_BUILD_WITH_QT_GUI
#include <QAction>
#endif // ifdef QLEANY_BUILD_WITH_QT_GUI
#include <QMutex>
#include <QObject>
#include <QThread>

using namespace Qleany;

namespace Qleany::Tools::UndoRedo
{
/*!
 * \ingroup UndoRedo
 * \brief A QObject encapsulating a threaded UndoRedoSystem that manages the
 * undo and redo command history
 * asynchronously.
 *
 * Implements a threaded undo-redo system by encapsulating the UndoRedoSystem
 * functionality in a
 * separate thread. This class ensures that undo and redo operations are
 * performed asynchronously without blocking the
 * main thread.
 */
class QLEANY_EXPORT ThreadedUndoRedoSystem : public QObject
{
    Q_OBJECT

  public:
    ThreadedUndoRedoSystem(QObject *parent, const Scopes &scopes);

    ~ThreadedUndoRedoSystem();

    static ThreadedUndoRedoSystem *instance();

    bool canUndo() const;

    bool canRedo() const;

    void undo();

    void redo();

    void push(UndoRedoCommand *command, const QString &commandScope, const QUuid &stackId = QUuid());
    void push(UndoRedoCommand *command, const QString &commandScope, const QUuid &stackId = QUuid()) const;

    void clear();

    void setUndoLimit(int limit);

    int undoLimit() const;

    QString undoText() const;

    QString redoText() const;

    QStringList undoRedoTextList() const;

    int currentIndex() const;

    void setCurrentIndex(int index);

    void setActiveStack(const QUuid &stackId = QUuid());

    QUuid activeStackId() const;

    void quitGracefully();

    QStringList queuedCommandTextListByScope(const QString &scopeFlagString) const;

    bool isRunning() const;

    int numberOfCommands() const;

#ifdef QLEANY_BUILD_WITH_QT_GUI
    QAction *createUndoAction(QObject *parent, const QString &prefix = QString()) const;
    QAction *createRedoAction(QObject *parent, const QString &prefix = QString()) const;
#endif // ifdef QLEANY_BUILD_WITH_QT_GUI

  Q_SIGNALS:

    /*!
     * \brief A signal that is emitted when the undo redo system state has
     *changed. Useful for the undo and redo
     * actions.
     */
    void stateChanged();

    /*!
     * \brief A signal that is emitted when a command results in an error.
     * actions.
     */
    void warningSent(Qleany::Error error);
    void errorSent(Qleany::Error error);

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     *start redoing.
     * actions.
     */
    void redoing(Scope scope, bool active);

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     *start undoing.
     * actions.
     */
    void undoing(Scope scope, bool active);

  private Q_SLOTS:

    void onUndoRedoSystemStateChanged();
    void onErrorSent(const Error &error);
    void onWarningSent(const Error &error);

  private:
    static ThreadedUndoRedoSystem *m_instance;
    mutable QMutex m_mutex;
    UndoRedoSystem *m_undoRedoSystemWorker = nullptr;
};
} // namespace Qleany::Tools::UndoRedo
