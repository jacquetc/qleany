// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_controller_export.h"
#include "undo_redo_system.h"
#include <QAction>
#include <QMutex>
#include <QObject>
#include <QThread>

using namespace FrontEnds;

namespace FrontEnds::Controller::UndoRedo
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
class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT ThreadedUndoRedoSystem : public QObject
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

    QAction *createUndoAction(QObject *parent, const QString &prefix = QString()) const;
    QAction *createRedoAction(QObject *parent, const QString &prefix = QString()) const;

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
    void warningSent(Error error);
    void errorSent(Error error);

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
} // namespace FrontEnds::Controller::UndoRedo